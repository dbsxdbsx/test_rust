use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::process::{Child, Command};
use std::sync::Arc;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcess, PROCESS_ALL_ACCESS};

pub static COMMAND_RUNNER: Lazy<Mutex<CommandRunner>> =
    Lazy::new(|| Mutex::new(CommandRunner::new().unwrap()));

struct SafeHandle(HANDLE);

unsafe impl Send for SafeHandle {}
unsafe impl Sync for SafeHandle {}
impl SafeHandle {
    fn new(handle: HANDLE) -> Self {
        Self(handle)
    }
}

impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_invalid() {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

struct JobHandle(Arc<SafeHandle>);

pub struct CommandRunner {
    job: JobHandle,
    tasks: Arc<Mutex<Vec<Child>>>,
}

impl CommandRunner {
    pub fn new() -> Result<Self> {
        let job = Self::create_job_and_assign_current_process()?;
        Ok(CommandRunner {
            job,
            tasks: Arc::new(Mutex::new(Vec::new())),
        })
    }

    fn create_job_and_assign_current_process() -> Result<JobHandle> {
        unsafe {
            let job = CreateJobObjectW(None, None)?;

            let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

            SetInformationJobObject(
                job,
                windows::Win32::System::JobObjects::JobObjectExtendedLimitInformation,
                &info as *const _ as *const _,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )?;

            let current_process = GetCurrentProcess();
            AssignProcessToJobObject(job, current_process)?;

            Ok(JobHandle(Arc::new(SafeHandle::new(job))))
        }
    }

    pub fn spawn(&self, command: &str) -> Result<u32> {
        let parts: Vec<&str> = command.split_whitespace().map(|part| part.trim()).collect();
        let (cmd_root, cmd_args) = parts.split_first().ok_or_else(|| anyhow!("无效的命令"))?;

        let mut command = Command::new(cmd_root);
        command.args(cmd_args);

        let child = command.spawn()?;
        let child_id = child.id();

        unsafe {
            let process = OpenProcess(PROCESS_ALL_ACCESS, false, child_id)?;
            AssignProcessToJobObject(self.job.0 .0, process)?;
            let _ = CloseHandle(process);
        }

        self.tasks.lock().push(child);

        Ok(child_id)
    }

    fn kill_tasks(&self) {
        let mut tasks = self.tasks.lock();
        for child in tasks.iter_mut() {
            if let Err(e) = child.kill() {
                eprintln!("终止子进程时出错: {}", e);
            }
        }
        tasks.clear();
        println!("所有子进程已终止");
    }

    pub fn stop(&self) -> Result<()> {
        self.kill_tasks();
        Ok(())
    }

    pub fn tasks_number(&self) -> usize {
        self.tasks.lock().len()
    }

    fn check_task_status(&self, pid: u32) -> Option<Result<bool, std::io::Error>> {
        let mut tasks = self.tasks.lock();
        tasks
            .iter_mut()
            .find(|child| child.id() == pid)
            .map(|child| child.try_wait().map(|status| status.is_none()))
    }

    pub fn is_task_running(&self, pid: u32) -> bool {
        match self.check_task_status(pid) {
            Some(Ok(running)) => running,
            _ => false,
        }
    }

    pub fn get_running_tasks_pids(&self) -> Vec<u32> {
        let tasks = self.tasks.lock();
        tasks.iter().map(|child| child.id()).collect()
    }
}

impl Drop for CommandRunner {
    fn drop(&mut self) {
        println!("CommandRunner 正在清理资源...");
        self.kill_tasks();
    }
}

pub fn test(app_last_secs: u64) -> Result<()> {
    let command_runner = COMMAND_RUNNER.lock();
    let child_pid = command_runner.spawn("./sing-box.exe run -c ./config.json")?;
    let child_pid2 = command_runner.spawn("ping -t 127.0.0.1")?;

    println!("Child process ID1: {}", child_pid);
    println!("Child process ID2: {}", child_pid2);
    println!("Current running tasks: {}", command_runner.tasks_number());

    // 检查进程状态
    for _ in 0..5 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!(
            "Process 1({child_pid}) is running: {}",
            command_runner.is_task_running(child_pid)
        );
        println!(
            "Process 2({child_pid2})  is running: {}",
            command_runner.is_task_running(child_pid2)
        );

        println!(
            "Running processes: {:?}",
            command_runner.get_running_tasks_pids()
        );
    }

    std::thread::sleep(std::time::Duration::from_secs(app_last_secs - 5)); // 减去上面的5秒
    command_runner.stop()?;
    println!("程序正常退出");

    Ok(())
}

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::process::{Child, Command};
use std::sync::Arc;
use std::{mem, ptr};
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::jobapi2::{AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject};
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcess};
use winapi::um::winnt::{
    HANDLE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};

pub static COMMAND_RUNNER: Lazy<Mutex<CommandRunner>> =
    Lazy::new(|| Mutex::new(CommandRunner::new().unwrap()));

struct JobHandle(Arc<HANDLE>);

impl Drop for JobHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(*Arc::as_ptr(&self.0));
        }
    }
}

pub struct CommandRunner {
    job: JobHandle,
    children: Arc<Mutex<Vec<Child>>>,
}

unsafe impl Send for CommandRunner {}
unsafe impl Sync for CommandRunner {}

impl CommandRunner {
    pub fn new() -> Result<Self> {
        let job = Self::create_job_and_assign_current_process()?;
        Ok(CommandRunner {
            job,
            children: Arc::new(Mutex::new(Vec::new())),
        })
    }

    fn create_job_and_assign_current_process() -> Result<JobHandle> {
        unsafe {
            let job = CreateJobObjectW(ptr::null_mut(), ptr::null());
            if job.is_null() {
                return Err(anyhow!("创建作业对象失败"));
            }

            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = mem::zeroed();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

            if SetInformationJobObject(
                job,
                winapi::um::winnt::JobObjectExtendedLimitInformation,
                &mut info as *mut _ as *mut _,
                mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as DWORD,
            ) == 0
            {
                CloseHandle(job);
                return Err(anyhow!("设置作业对象信息失败"));
            }

            let current_process = GetCurrentProcess();
            if AssignProcessToJobObject(job, current_process) == 0 {
                CloseHandle(job);
                return Err(anyhow!("将当前进程分配到作业失败"));
            }

            Ok(JobHandle(Arc::new(job)))
        }
    }

    pub fn spawn(&self, command: &str, args: &[&str]) -> Result<u32> {
        let child = Command::new(command).args(args).spawn()?;

        let child_id = child.id();

        unsafe {
            let process = OpenProcess(winapi::um::winnt::PROCESS_ALL_ACCESS, 0, child_id);
            if process.is_null() {
                return Err(anyhow!("Failed to open child process"));
            }
            AssignProcessToJobObject(*Arc::as_ptr(&self.job.0), process);
            CloseHandle(process);
        }

        self.children.lock().push(child);

        Ok(child_id)
    }

    fn kill_children(&self) {
        let mut children = self.children.lock();
        for child in children.iter_mut() {
            if let Err(e) = child.kill() {
                eprintln!("终止子进程时出错: {}", e);
            }
        }
        children.clear();
        println!("所有子进程已终止");
    }

    pub fn stop(&self) -> Result<()> {
        self.kill_children();
        Ok(())
    }

    pub fn task_number(&self) -> usize {
        self.children.lock().len()
    }
}

impl Drop for CommandRunner {
    fn drop(&mut self) {
        println!("CommandRunner 正在清理资源...");
        self.kill_children();
    }
}

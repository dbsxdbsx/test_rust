use ctrlc;
use std::error::Error;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct ThreadLoop {
    children: Arc<Mutex<Vec<Child>>>,
}

impl ThreadLoop {
    fn new() -> Self {
        let children: Arc<Mutex<Vec<Child>>> = Arc::new(Mutex::new(Vec::new()));

        // 设置Ctrl+C处理
        let children_clone = Arc::clone(&children);
        ctrlc::set_handler(move || {
            println!("捕获到Ctrl+C, 终止所有子进程...");
            Self::kill_children(&children_clone);
            std::process::exit(0);
        })
        .expect("设置Ctrl+C处理失败");

        // 创建子进程
        let children_clone = Arc::clone(&children);
        let child = Command::new("ping")
            .args(&["-t", "127.0.0.1"])
            .spawn()
            .expect("无法启动子进程");

        children_clone.lock().unwrap().push(child);

        // 创建监控线程
        let children_clone = Arc::clone(&children);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(100));
                let mut children = children_clone.lock().unwrap();
                children.retain_mut(|child| match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("子进程退出，状态: {:?}", status);
                        false
                    }
                    Ok(None) => true,
                    Err(e) => {
                        eprintln!("检查子进程状态时出错: {}", e);
                        false
                    }
                });
                if children.is_empty() {
                    break;
                }
            }
            println!("所有子进程已退出");
        });

        ThreadLoop { children }
    }

    fn kill_children(children: &Arc<Mutex<Vec<Child>>>) {
        let mut children = children.lock().unwrap();
        for child in children.iter_mut() {
            if let Err(e) = child.kill() {
                eprintln!("终止子进程时出错: {}", e);
            }
        }
        println!("所有子进程已终止");
    }
}

impl Drop for ThreadLoop {
    fn drop(&mut self) {
        Self::kill_children(&self.children);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _t = ThreadLoop::new();
    println!("ThreadLoop已创建");

    // 模拟程序运行一段时间
    thread::sleep(Duration::from_secs(5));

    // 正常退出或者取消下面的注释来模拟panic
    // panic!("模拟程序panic");

    println!("程序正常退出");
    Ok(())
}

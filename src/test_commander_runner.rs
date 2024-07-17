use std::thread;
use std::time::Duration;

use command_runner::{self, CommandRunner, CommandStatus};

pub fn test() {
    let ping_count_option = if cfg!(target_os = "windows") {
        "-n"
    } else {
        "-c"
    };
    let ping_num = 2;
    // let ping_command = format!("ping {} {} google.com", ping_count_option, ping_num);
    let ping_command = "./customized_app.exe";
    let mut runner =
        CommandRunner::new(&ping_command, 10000).expect("Failed to create CommandRunner");

    let mut output_count = 0;
    loop {
        if let Some(output) = runner.get_output() {
            println!("Got Output: {}", output);
            output_count += 1;
        }
        let status = runner.get_status();
        println!("Current status: {:?}", status);
        if status == CommandStatus::Terminated {
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }

    assert!(output_count > ping_num, "No output received");
    assert_eq!(runner.get_status(), CommandStatus::Terminated);
}

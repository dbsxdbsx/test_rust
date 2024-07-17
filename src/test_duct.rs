use duct::cmd;
use encoding_rs::GB18030;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::prelude::*;
use std::io::BufReader;

pub fn test() {
    let command = cmd!(r"C:\Windows\System32\cmd.exe", "/C", "ping -t 127.0.0.1").stdout_capture();
    let output = command.reader().unwrap();

    // 使用GB18030解码器包装输出
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(GB18030))
        .build(output);

    let reader = BufReader::new(decoder);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("收到输出: {}", line);
            }
            Err(e) => eprintln!("读取行时出错: {}", e),
        }
    }
    // let output = cmd!(r"C:\Windows\System32\cmd.exe", "/C", "echo Hello, world!").read().unwrap();
    // println!("{}", output);
}

use std::env;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 解析命令行参数
    let mut port: Option<u16> = None;
    for arg in args.iter() {
        if arg.starts_with("--port=") {
            if let Ok(value) = arg.split("=").nth(1).unwrap().parse() {
                port = Some(value);
            }
        }
    }

    // 打印参数值
    if let Some(port) = port {
        println!("端口号：{}", port);
    } else {
        println!("未指定端口号");
    }
}
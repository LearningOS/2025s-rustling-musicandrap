// exercises/enums/enums2.rs

#[derive(Debug)]
enum Message {
    // 定义四种不同形式的枚举变体
    Move { x: i32, y: i32 },         // 结构体风格的变体
    Echo(String),                    // 元组风格的变体（单个String）
    ChangeColor(i32, i32, i32),      // 元组风格的变体（三个i32）
    Quit,                            // 无数据的简单变体
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
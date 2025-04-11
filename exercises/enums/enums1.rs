// exercises/enums/enums1.rs

#[derive(Debug)]
enum Message {
    // 定义四个枚举变体
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
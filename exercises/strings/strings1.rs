// exercises/strings/strings1.rs

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()  // 将 &str 转换为 String
}
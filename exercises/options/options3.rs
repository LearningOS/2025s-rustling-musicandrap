struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {  // 使用引用匹配，避免移动所有权
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // 现在这行可以正常使用了
}
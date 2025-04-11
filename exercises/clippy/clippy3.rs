fn main() {
    // 1. 正确处理Option - 改为更合理的处理方式
    let my_option: Option<()> = None;
    match my_option {
        Some(_) => println!("Got Some value"),
        None => println!("Expected Some value but got None"), // 改为非panic处理
    }

    // 2. 修复数组语法并改进格式
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,  // 添加了缺失的逗号
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 修复向量清除并添加类型注释
    let mut my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 4. 修复值交换并移除不必要的mut
    let mut value_a = 45;
    let mut value_b = 66;
    // 更清晰的交换注释
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
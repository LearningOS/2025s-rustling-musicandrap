fn main() {
    // 1. Fixed Option handling
    let my_option: Option<()> = None;
    if my_option.is_none() {
        panic!("Expected Some value but got None");
    }

    // 2. Fixed array syntax (missing comma)
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. Fixed vector clearing
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 4. Fixed value swapping
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
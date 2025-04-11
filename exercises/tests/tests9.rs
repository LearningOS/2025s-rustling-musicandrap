// 声明要链接的外部Rust函数
extern "Rust" {
    #[link_name = "my_demo_function"]  // 直接使用函数名
    fn my_demo_function(a: u32) -> u32;
    
    #[link_name = "my_demo_function"]  // 别名也指向同一个函数
    fn my_demo_function_alias(a: u32) -> u32;
}

// 使用蛇形命名法的模块
mod foo {
    #[no_mangle]  // 禁止名称修饰
    pub fn my_demo_function(a: u32) -> u32 {
        a  // 简单返回输入值
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            // 测试两个函数调用
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}
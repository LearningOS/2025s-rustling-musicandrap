#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),  // 使用 Box 包装递归类型
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil  // 空列表就是 Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil))  // 创建包含一个元素的列表
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
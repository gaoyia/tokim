fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    add(1,1);
}

// 添加一个测试模块
#[cfg(test)]
mod tests {
    // 导入当前作用域的所有模块
    use super::*;

    // 添加一个测试函数
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
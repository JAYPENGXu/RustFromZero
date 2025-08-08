pub fn length(a:&str) -> usize {
    a.len()
}

// 所有 String 都可以用 .as_str() 转成 &str
// 但反过来 &str 却不能自动变成 &String
pub fn comsume_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> Iter,    // F 是一个闭包，接受 i32，返回 Iter 类型
    Iter: Iterator<Item = T>, // Iter 是一个 Iterator，Item 是 T 类型
    T: std::fmt::Debug,       // T 实现了 Debug trait
{
    // 根据 F 的类型，f(10) 返回 iterator，所以可以用 for 循环
    for item in f(10) {
        println!("{:?}", item); // item 实现了 Debug trait，所以可以用 {:?} 打印
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_iterator() {
        // 不会 panic 或者出错
        comsume_iterator(|i| 0..i)
    }
}

#[allow(unused_variables, unused_mut)]

fn main() {
    let mut data = vec![1, 2, 3];

    // Rust 下，根据所有权规则，你无法同时拥有多个可变引用
    // for item in data.iter_mut() {
    //     data.push(*item + 1);
    // }
}

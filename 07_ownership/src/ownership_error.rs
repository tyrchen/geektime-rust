fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1: {}", sum(data1));
    // 下面两句无法编译通过
    // println!("data1: {:?}", data1);
    // println!("sum of data: {}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().sum()
}

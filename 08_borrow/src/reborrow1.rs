fn main() {
    let mut data = vec![1, 2, 3, 4];
    let b = &mut data;
    println!("addr of the ref b: {:p}", &b);
    println!("sum of data1: {}", sum(b));
    // ok
    println!("{:?}", b);
}

fn sum(v: &mut Vec<i32>) -> i32 {
    println!("addr of the ref v: {:p}", &v);
    v.iter().sum()
}

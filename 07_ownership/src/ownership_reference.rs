fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // what is the addr of value, and what about reference?
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // what about addrs for items in heap?
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &[u32]) -> u32 {
    // would addr of value change? would addr of reference change?
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().sum()
}

#[allow(dead_code)]
struct Data<T> {
    inner: T,
}

fn process_data<T: Sized>(_data: Data<T>) {
    todo!();
}

#[allow(dead_code)]
struct UnsizedData<T: ?Sized> {
    inner: T,
}

// 无法编译通过，函数的参数必须是编译时大小确定的
// fn process_unsized_data<T>(data: UnsizedData<T>)
// where
//     T: ?Sized,
// {
//     todo!();
// }

#[allow(unused_variables)]
fn main() {
    let v = Data { inner: 0 };
    process_data(v);
    let v = UnsizedData { inner: 0 };
    // process_unsized_data(v);
}

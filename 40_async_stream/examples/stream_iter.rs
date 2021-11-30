use futures::prelude::*;

#[tokio::main]
async fn main() {
    let mut st = stream::iter(1..10)
        .filter(|x| future::ready(x % 2 == 0))
        .map(|x| x * x);

    println!("Type of stream: {}", get_type_name(&st));
    while let Some(x) = st.next().await {
        println!("Got item: {}", x);
    }
}

fn get_type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

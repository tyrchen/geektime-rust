use macros::query;

fn main() {
    query!(SELECT * FROM users WHERE age > 10);
    hello()
}

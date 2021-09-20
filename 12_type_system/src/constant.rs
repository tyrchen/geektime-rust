const PI: f64 = std::f64::consts::PI;
static E: f32 = std::f32::consts::E;

fn main() {
    const V: u32 = 10;
    static V1: &str = "hello";
    println!("PI: {}, E: {}, V {}, V1: {}", PI, E, V, V1);
}

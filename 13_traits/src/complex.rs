use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

// 对 Complex 类型的实现
impl Add for Complex {
    type Output = Self;

    // 注意 add 第一个参数是 self，会移动所有权
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

// 如果不想移动所有权，可以为 &Complex 实现 add，这样可以做 &c1 + &c2
impl Add for &Complex {
    // 注意返回值不应该是 Self 了，因为此时 Self 是 &Complex
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

// 因为 Add<Rhs = Self> 是个泛型 trait，我们可以为 Complex 实现 Add<f64>
impl Add<f64> for &Complex {
    type Output = Complex;

    // rhs 现在是 f64 了
    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2f64, 3.0);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 5.0);
    println!("{:?}", c1 + c2);
    // c1, c2 已经被移动，所以下面这句无法编译
    // println!("{:?}", c1 + c2);
}

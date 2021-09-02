const MY_PI: f64 = std::f64::consts::PI;

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

#[allow(dead_code)]
enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area(),
        }
    }
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.r * self.r * MY_PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c) / 2.0;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }
}

fn main() {
    let rec = Rectangle { a: 10.0, b: 5.0 };
    let shape = Shape::Rec(rec);
    println!("area is: {}", shape.area());
}

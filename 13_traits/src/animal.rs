struct Cat;
struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn name(animal: impl Animal) -> &'static str {
    animal.name()
}

fn main() {
    let cat = Cat;
    println!("cat: {}", name(cat));
}

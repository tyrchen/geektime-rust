const PI: f64 = 3.1415926;
static V: Vec<u8> = Vec::new();

#[allow(dead_code)]
fn where_is_pi() {
    let r = 10f64;
    println!(
        "addr(r): {:p}, addr(PI): {:p}, addr(PI1): {:p}, area is: {}",
        &r,
        &PI,
        &V,
        PI * r * r
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_where_is_pi() {
        where_is_pi();
    }

    #[test]
    fn test_struct() {
        #[derive(Debug)]
        struct Marker;

        #[derive(Debug)]
        struct Color(u8, u8, u8);

        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let marker = Marker {};
        let color = Color(128, 128, 128);
        let person = Person {
            name: "Tyr".into(),
            age: 18,
        };

        println!(
            "marker: {:?}, color: {:?}, person: {:?}",
            marker, color, person
        );
    }

    #[test]
    fn test_enum() {
        #[derive(Debug)]
        #[allow(dead_code)]
        enum MyOption<T> {
            Some(T),
            None,
        }

        #[derive(Debug)]
        #[allow(dead_code)]
        enum Status {
            Ok = 0,
            BadName = 1,
            NotFound = 2,
            Internal = 3,
        }

        let opt = MyOption::Some("hello");
        let status = Status::NotFound;
        println!("opt is {:?}, status is: {:?}", opt, status);
    }
}

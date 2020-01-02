use json_serializable::*;

fn main() {
    // Sample struct for serialize
    serializable!(
        pub struct Person {
            name: String,
            age: u8,
        }
    );

    impl Person {
        pub fn new(name: String, age: u8) -> Self {
            Self {
                name: name,
                age: age,
            }
        }
    }

    let person = Person::new("furuhama".to_string(), 30);

    println!("{}", person.serialize().unwrap());
}

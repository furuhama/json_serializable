use json_serializable::*;

fn main() {
    let person = Person::new("furuhama".to_string(), 30);

    println!("{}", person.serialize().unwrap());
}

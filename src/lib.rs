pub trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, ()>;
}

// Sample struct for serialize
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }
}

// This impl section should be generated by macro
impl Serializable for Person {
    fn serialize(&self) -> Result<Vec<u8>, ()> {
        let mut result: Vec<u8> = "{".as_bytes().to_vec();

        result.append(&mut "\"Person\":{".as_bytes().to_vec());
        result.append(&mut format!("\"name\":\"{}\",", self.name).as_bytes().to_vec());
        result.append(&mut format!("\"age\":{}", self.age).as_bytes().to_vec());
        result.append(&mut "}".as_bytes().to_vec());

        result.append(&mut "}".as_bytes().to_vec());

        Ok(result)
    }
}

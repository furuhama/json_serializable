#[macro_export]
macro_rules! serializable {
    (
        struct $name:ident {
            $($attr:ident : $type:ty,)*
        }
    ) => {
        struct $name {
            $(
                $attr: $type,
            )*
        }

        serialize_internal!($name { $($attr: $type,)* });
    };
    (
        pub struct $name:ident {
            $($attr:ident : $type:ty,)*
        }
    ) => {
        pub struct $name {
            $(
                $attr: $type,
            )*
        }

        serialize_internal!($name { $($attr: $type,)* });
    };
}

#[macro_export]
macro_rules! serialize_internal {
    (
        $name:ident {
            $($attr:ident : $type:ty,)*
        }
    ) => {
        impl Serializable for $name {
            fn serialize(&self) -> Result<String, ()> {
                let mut result = String::from("{");
                result.push_str(&format!("\"{}\":", stringify!($name)));
                self.serialize_fields(&mut result);
                result.push_str("}");

                Ok(result)
            }

            fn serialize_fields(&self, result: &mut String) {
                result.push_str("{");
                $(
                    result.push_str(&format!("\"{}\":\"{}\",", stringify!($attr), self.$attr));
                )*
                result.push_str("}");
            }
        }
    }
}

pub trait Serializable {
    fn serialize(&self) -> Result<String, ()>;
    fn serialize_fields(&self, outer: &mut String);
}

#[cfg(test)]
mod tests {
    use super::Serializable;

    serializable! {
        struct TestPerson {
            name: String,
            age: u8,
        }
    }

    #[test]
    fn test() {
        let person = TestPerson { name: "Witcher".to_string(), age: 10 };

        let serialized_person = person.serialize().unwrap();

        assert_eq!(serialized_person, "{\"TestPerson\":{\"name\":\"Witcher\",\"age\":\"10\",}}");
    }
}

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

// Sample struct for serialize
serializable!(
    pub struct Person {
        name: String,
        age: u8,
    }
);

pub trait Serializable {
    fn serialize(&self) -> Result<String, ()>;
    fn serialize_fields(&self, outer: &mut String);
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }
}

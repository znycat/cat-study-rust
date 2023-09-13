mod protopackage;

pub use protopackage::*;

use self::person::{PhoneNumber, PhoneType};

impl Person {
    pub fn new(
        name: impl Into<String>,
        id: i32,
        email: impl Into<String>,
        phones: &[PhoneNumber],
    ) -> Person {
        Self {
            name: name.into(),
            id,
            email: email.into(),
            phones: phones.to_vec(),
            ..Default::default()
        }
    }
}

impl PhoneNumber {
    pub fn new(number: impl Into<String>, r#type: PhoneType) -> PhoneNumber {
        Self {
            number: number.into(),
            r#type: r#type as i32,
        }
    }
}

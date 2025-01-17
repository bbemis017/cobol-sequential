use lombok::{AllArgsConstructor, Getter};
use std::fmt;

/// A FieldDefinition defines a copybook field.
/// This contains it's level, label, and data type.
#[derive(AllArgsConstructor, Getter, Debug, Clone, PartialEq)]
pub struct FieldDefinition {
    // Field's defined level in the copybook. This is the number the comes before every field
    // name in the copybook.
    level: u32,

    // The Field's label is defined by the copybook and used to reference the field.
    label: String,

    data_type: String, //TODO should be an enum
}

impl fmt::Display for FieldDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldDefinition level={}, label={}, dataType={}",
            self.get_level(),
            self.get_label(),
            self.get_data_type()
        )
    }
}

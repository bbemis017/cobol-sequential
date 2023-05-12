//! Structures used to model Field values from copybook.

use std::collections::HashMap;

use lombok::{AllArgsConstructor, Getter};

// CopybookValueEnum contains meta-data about potential values for a Copybook Field. Typically,
// these listed values are not exhaustive but can sometimes provide additional context to a
// copybook or copybook field.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValueEnum {
    // A FieldValue is used for a field with a single value. These are typically defined in COBOL
    // as something like "01 FIELDNAME PIC X(2) VALUE 'AA'"
    FieldValue(FieldValue),

    // A FieldValueMap is used to map labels to potential field values. These are used for COBOL
    // level 88 statements. For simplicity we should merge level 88 statements together into this
    // map.
    FieldValueMap(FieldValueMap),
}

// A single field value defined with the value clause.
#[derive(Debug, Clone, PartialEq, Getter, AllArgsConstructor)]
pub struct FieldValue {
    // The value span contains the raw text for the field value. Which may contain a value, a
    // constant like SPACES, or a range. It may make sense to expand on this later and actually
    // model the different data types for values in rust. But since it's not essential for
    // parsing sequential files I'm going to leave that out for now.
    value_span: String,
}

// A Map of labels to potential field values. These values are collected from level 88 statements
// which may look like this in the COBOL
// ```
// 01 STATES PIC X(2).
//    88 ILLINOIS 'IL'.
//    88 INDIANA  'IN'.
// ```
// Typically level 88 statements are not exhaustive.
#[derive(Debug, Clone, Getter)]
pub struct FieldValueMap {
    // Maps label to field value.
    label_to_value_map: HashMap<String, FieldValue>,
}

impl PartialEq for FieldValueMap {
    //TODO test implementation
    fn eq(&self, other: &Self) -> bool {
        if self.get_label_to_value_map().len() != other.get_label_to_value_map().len() {
            return false;
        }

        for (label, field_value) in self.get_label_to_value_map().into_iter() {
            match other.get_label_to_value_map().get(label) {
                Some(other_field_value) => {
                    if field_value != other_field_value {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }
}

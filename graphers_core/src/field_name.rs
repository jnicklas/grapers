use std::fmt;

#[derive(Clone, Debug)]
pub struct FieldName(pub String);

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
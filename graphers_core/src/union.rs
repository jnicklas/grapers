use super::Type;

#[derive(Debug, Eq, PartialEq)]
pub struct Union {
    types: Vec<Type>,
}

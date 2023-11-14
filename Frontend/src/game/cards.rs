#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardCell {
    Empty,
    Card(String)
}

pub trait CardClass {
    fn get_class(&self) -> &str;
}
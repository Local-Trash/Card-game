pub enum CardCell {
    Empty,
    Card(&str)
}

pub trait CardClass {
    fn get_class(&self) -> &str;
}
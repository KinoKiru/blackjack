pub struct Card {
    pub short_name: String,
    pub value: u8,
}

//override tostring with own implementation
impl ToString for Card {
    fn to_string(&self) -> String {
        return self.short_name;
    }
}

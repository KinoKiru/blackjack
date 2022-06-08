#[derive(Debug)]

pub struct Card {
    pub short_name: String,
    pub value: u8,
}

//override to_string with own implementation
//TODO make a to string
// impl ToString for Card {
//     fn to_string(&self) -> String {
//         return self.short_name;
//     }
// }

#[derive(Debug, Clone)]
pub struct Card {
    pub short_name: String,
    pub value: u8,
}

//override to_string with own implementation
//TODO make a to string
// impl Join<String> for Card {
//     type Output = String;
//     fn join(slice: &Self, sep: Separator) -> Self::Output {
//         return slice.short_name.clone() + sep;
//     }
// }

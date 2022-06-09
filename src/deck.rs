use crate::card::Card;
use rand::Rng;
// For objects
pub struct Deck {
    pub cards: Vec<Card>,
}

//impementation for deck (add a function)
impl Deck {
    //constructor
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![];
        //get the value from 0-52 and is a u8
        for value in 0..53_u8 {
            let mut short_name = if value < 13 {
                "H"
            } else if value < 26 {
                "S"
            } else if value < 39 {
                "D"
            } else {
                "C"
            }
            .to_string();
            let value = (value % 13) + 1;

            short_name += match value {
                1 => "A".to_owned(),
                11 => "J".to_owned(),
                12 => "Q".to_owned(),
                13 => "K".to_owned(),
                _ => (value).to_string(),
            }
            .as_str();
            cards.push(Card { value, short_name });
        }
        cards.push(Card {
            short_name: String::from("JK"),
            value: 0,
        });
        cards.push(Card {
            short_name: String::from("JK"),
            value: 0,
        });

        Deck { cards }
    }

    //gives instance to the function
    pub fn pick_random(&mut self) -> Card {
        let index = rand::thread_rng().gen_range(0..self.cards.len());
        self.cards.remove(index)
    }

    //TODO make backman pickmode
}
// For single values
//pub type Deck = Vec<Card>;

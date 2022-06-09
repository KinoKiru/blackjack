use crate::model::card::Card;
use rand::Rng;
// For objects
pub struct Deck {
    cards: Vec<Card>,
}

//implementation for deck (add a function)
impl Deck {
    //constructor
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![];
        //get the value from 0-52 and is a u8
        for value in 0..52_u8 {
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
            let mut value = value;
            if value > 10 {
                value = 10;
            }

            cards.push(Card { value, short_name });
        }
        let mut deck = Deck { cards };
        deck.shuffle();

        return deck;
    }

    //gives instance to the function
    pub fn pick_random(&mut self) -> Card {
        let index = rand::thread_rng().gen_range(0..self.cards.len());
        self.cards.remove(index)
    }

    //TODO make backman pickmode

    pub fn black_man_mode(&mut self) -> Card {
        let index = self.cards.iter().position(|card| card.value >= 10).unwrap();

        self.cards.remove(index)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _index in 0..100 {
            let rng_1 = rng.gen_range(0..self.cards.len());
            let rng_2 = rng.gen_range(0..self.cards.len());

            let saved = self.cards[rng_1].clone();

            self.cards[rng_1] = self.cards[rng_2].clone();
            self.cards[rng_2] = saved;
        }
        //self.cards.shuffle(&mut rng);
    }
}
// For single values
//pub type Deck = Vec<Card>;

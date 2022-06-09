use super::{card::Card, deck::Deck};

// one-way linking to deck
pub struct Dealer {
    name: String,
    pub hand: Vec<Card>,
}

impl Dealer {
    pub fn new(name: String, deck: &mut Deck) -> Self {
        let hand: Vec<Card> = vec![];

        //created a new instance
        let mut dealer = Dealer { hand, name };

        dealer.draw(deck);

        return dealer;
    }

    pub fn draw(&mut self, deck: &mut Deck) {
        if self.name == "Jerome" {
            self.hand.push(deck.black_man_mode());
        } else {
            self.hand.push(deck.pick_random());
        }
    }
}

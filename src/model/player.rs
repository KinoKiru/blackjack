use super::{card::Card, deck::Deck};

#[derive(Clone)]

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String, deck: &mut Deck) -> Self {
        let hand: Vec<Card> = vec![];

        let mut player = Player { name, hand };
        player.hit(deck);
        player.hit(deck);

        return player;
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        self.hand.push(deck.pick_random());
    }
}

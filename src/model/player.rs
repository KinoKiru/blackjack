use super::{card::Card, deck::Deck};

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub has_standed: bool,
}

impl Player {
    pub fn new(name: String, deck: &mut Deck) -> Self {
        let hand: Vec<Card> = vec![];
        let has_standed = false;

        let mut player = Player {
            name,
            hand,
            has_standed,
        };
        player.hit(deck);
        player.hit(deck);

        return player;
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        self.hand.push(deck.pick_random());
    }
}

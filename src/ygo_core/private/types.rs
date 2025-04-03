use crate::ygo_core::card_db::CardName;
use crate::ygo_core::private::state::State;

#[derive(Clone, Debug)]
pub struct Card {
    pub name: CardName,
    pub effect: Box<fn(State) -> State>,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
}
impl Player {
    pub fn new(deck: Vec<Card>) -> Self {
        let (hand, deck_after_draw) = deck.split_at(1);
        Self {
            deck: deck_after_draw.to_vec(),
            hand: hand.to_vec(),
        }
    }
}

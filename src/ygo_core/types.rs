use crate::ygo_core::card_db::Card;
use crate::ygo_core::game::{MAX_DECK_SIZE, MIN_DECK_SIZE};

#[derive(Debug, Clone)]
pub struct Deck(Vec<Card>);
impl Deck {
    pub fn new(cards: Vec<Card>) -> Result<Self, String> {
        let len = cards.len();
        if len < MIN_DECK_SIZE || len > MAX_DECK_SIZE {
            return Err(format!(
                "Deck must contain between {} and {} cards. Got: {}",
                MIN_DECK_SIZE, MAX_DECK_SIZE, len
            ));
        }
        Ok(Deck(cards))
    }

    pub fn cards(&self) -> &[Card] {
        &self.0
    }
}

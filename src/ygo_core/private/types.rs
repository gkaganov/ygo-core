use crate::ygo_core::card_db::Card;
use crate::ygo_core::game::INITIAL_HAND_SIZE;
use crate::ygo_core::types::Deck;

#[derive(Clone, Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub deck: Deck,
}
impl Player {
    pub fn new(deck: Deck) -> Self {
        let (hand, deck_after_draw) = deck.cards().split_at(INITIAL_HAND_SIZE);
        Self {
            deck: Deck::new(deck_after_draw.to_vec()).unwrap(),
            hand: hand.to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    ActivateCardInHand(usize),
    ActivateCardInMainMonsterZone(usize),
    ActivateCardInExtraMonsterZone(usize),
    ActivateCardInSpellTrapZone(usize),
    ActivateCardInFieldSpellZone(usize),
    ActivateCardInGraveyard(usize),
}

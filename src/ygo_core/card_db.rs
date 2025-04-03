use crate::ygo_core::private::card_text_logic::{dark_hole, pot_of_greed};
use crate::ygo_core::private::types::Card;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CardName {
    PotOfGreed,
    DarkHole,
}

pub static CARDS: LazyLock<HashMap<CardName, Card>> = LazyLock::new(|| {
    let cards = [
        Card {
            name: CardName::PotOfGreed,
            effect: Box::new(pot_of_greed),
        },
        Card {
            name: CardName::DarkHole,
            effect: Box::new(dark_hole),
        },
    ];
    HashMap::from(cards.map(|card| (card.name.clone(), card)))
});

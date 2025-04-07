use crate::ygo_core::state::State;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Debug)]
pub struct Card {
    pub name: CardName,
    // source_player_index: usize, state: State
    pub effect: Box<fn(usize, State) -> State>,
}

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

fn pot_of_greed(source_player_index: usize, state: State) -> State {
    state.draw_cards(source_player_index, 2)
}

fn dark_hole(source_player_index: usize, state: State) -> State {
    state
}

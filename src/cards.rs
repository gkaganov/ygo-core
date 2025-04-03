use crate::State;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Card {
    name: CardName,
    pub effect: Box<fn(State) -> State>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CardName {
    PotOfGreed,
    DarkHole,
}

fn pot_of_greed(state: State) -> State {
    draw_cards(state, 2)
}

fn dark_hole(state: State) -> State {
    state
}

fn draw_cards(state: State, amount: usize) -> State {
    let mut players = state.players.clone();
    let (hand, deck_after_draw) = players[state.turn_player].deck.split_at(amount);
    players[state.turn_player].hand.append(&mut hand.to_vec());
    players[state.turn_player].deck = deck_after_draw.to_vec();

    State {
        players,
        turn_player: state.turn_player,
    }
}

// lazily initializes the CARDS database at runtime
lazy_static! {
    pub static ref CARDS: HashMap<CardName, Card> = {
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
    };
}

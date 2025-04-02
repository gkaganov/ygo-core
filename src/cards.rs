use crate::State;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Card {
    display_name: String,
    effect: Box<fn(State) -> State>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CardName {
    PotOfGreed,
    DarkHole,
}

fn pot_of_greed(state: State) -> State {
    draw_cards(state, 2)
}

fn draw_cards(state: State, amount: usize) -> State {
    let mut players = state.players.clone();
    let (hand, deck_after_draw) = players[state.turn_player].deck.split_at(amount);
    players[state.turn_player].hand = hand.to_vec();
    players[state.turn_player].deck = deck_after_draw.to_vec();

    State {
        players,
        turn_player: state.turn_player,
    }
}

// lazily initializes the CARDS database at runtime
lazy_static! {
    static ref CARDS: HashMap<CardName, Card> = {
        HashMap::from([(
            CardName::PotOfGreed,
            Card {
                display_name: String::from("Pot of Greed"),
                effect: Box::new(pot_of_greed),
            },
        )])
    };
}

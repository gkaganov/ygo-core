use crate::ygo_core::card_db::CardName;
use crate::ygo_core::game::{Deck, Game};
use ygo_core::card_db::CARDS;

mod ygo_core;

fn main() {
    let deck1 = Deck::new(vec![
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::DarkHole).unwrap().clone(),
    ])
    .unwrap();

    let deck2 = Deck::new(vec![
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::DarkHole).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::DarkHole).unwrap().clone(),
    ])
    .unwrap();

    let game = Game::new([deck1, deck2]);
    println!("Initialized: {:#?}", game);

    let legal_actions = game.legal_actions();
    println!("Legal actions: {:#?}", legal_actions);

    let my_action = legal_actions.get(0).unwrap();
    println!("My action: {:#?}", legal_actions);

    let game_after_action = game.take_action(my_action);
    println!("Game after action: {:#?}", game_after_action);
}

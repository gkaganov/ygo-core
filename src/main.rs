use crate::ygo_core::card_db::CardName;
use crate::ygo_core::core::Core;
use ygo_core::card_db::CARDS;

mod ygo_core;

fn main() {
    let deck1 = vec![
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::DarkHole).unwrap().clone(),
    ];
    let deck2 = vec![
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::DarkHole).unwrap().clone(),
        CARDS.get(&CardName::PotOfGreed).unwrap().clone(),
        CARDS.get(&CardName::DarkHole).unwrap().clone(),
    ];
    let core = Core::new([deck1, deck2]);
    println!("Initialized {:#?}", core);
    let core_after_pot_of_greed = core.activate_card(0);
    println!("Activated Pot of Greed {:#?}", core_after_pot_of_greed);
}

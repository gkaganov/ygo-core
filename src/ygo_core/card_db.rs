use crate::ygo_core::card_db::CardName::PotOfGreed;
use crate::ygo_core::card_db::CardTemplate::{Monster, Spell};
use crate::ygo_core::card_db::SpellType::Normal;
use crate::ygo_core::state::State;
use std::collections::HashMap;
use std::sync::LazyLock;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum CardTemplate {
    Monster { name: CardName, data: MonsterType },
    Spell { name: CardName, data: SpellType },
}
impl CardTemplate {
    pub fn name(&self) -> &CardName {
        match self {
            Monster { name, .. } => name,
            Spell { name, .. } => name,
        }
    }
}

#[derive(Clone, Debug)]
pub enum MonsterType {
    Normal {
        name: CardName,
        level: u8,
        atk: u32,
        def: u32,
    },
    Effect {
        name: CardName,
        level: u8,
        atk: u32,
        def: u32,
        effect: Box<fn(usize, State) -> State>,
    },
    Fusion,
    Xyz,
    Link,
}

#[derive(Clone, Debug)]
pub enum SpellType {
    Normal {
        name: CardName,
        effect: Box<fn(usize, State) -> State>,
    },
    QuickPlay,
}

#[derive(Clone, Debug)]
pub enum CardLocation {
    Deck(usize),
    ExtraDeck(usize),
    Hand(usize),
    Field(usize),
    Graveyard(usize),
    Banishment(usize),
}

#[derive(Clone, Debug)]
pub struct CardInstance {
    pub id: Uuid,
    pub name: CardName,
    pub owner_index: usize,
    pub controller_index: usize,
    pub location: CardLocation,
}
impl CardInstance {
    pub fn new(name: CardName, owner_index: usize, location: CardLocation) -> Result<Self, String> {
        let card_template = CARDS.get(&name).unwrap();
        let valid_location = match (card_template, &location) {
            (CardTemplate::Monster(MonsterType::Fusion, ..), CardLocation::Deck(_)) => false,
            (CardTemplate::Monster(MonsterType::Xyz, ..), CardLocation::Deck(_)) => false,
            (CardTemplate::Monster(MonsterType::Link, ..), CardLocation::Deck(_)) => false,

            (CardTemplate::Monster(MonsterType::Fusion, ..), CardLocation::Hand(_)) => false,
            (CardTemplate::Monster(MonsterType::Xyz, ..), CardLocation::Hand(_)) => false,
            (CardTemplate::Monster(MonsterType::Link, ..), CardLocation::Hand(_)) => false,

            (CardTemplate::Monster(MonsterType::Normal, ..), CardLocation::ExtraDeck(_)) => false,
            (CardTemplate::Monster(MonsterType::Effect, ..), CardLocation::ExtraDeck(_)) => false,

            _ => true,
        };
        if !valid_location {
            return Err(String::from("location not ok"));
        }
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            owner_index,
            controller_index,
            location,
        })
    }

    pub fn template(&self) -> &CardTemplate {
        CARDS.get(&self.name).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CardName {
    PotOfGreed,
    DarkHole,
}

pub static CARDS: LazyLock<HashMap<CardName, CardTemplate>> = LazyLock::new(|| {
    let cards = [
        Spell(Normal {
            name: PotOfGreed,
            effect: Box::new(pot_of_greed),
        }),
        Spell(Normal {
            name: PotOfGreed,
            effect: Box::new(pot_of_greed),
        }),
    ];
    HashMap::from(cards.map(|card| (card.name.clone(), card)))
});

fn pot_of_greed(source_player_index: usize, state: State) -> State {
    state.draw_cards(source_player_index, 2)
}

fn dark_hole(source_player_index: usize, state: State) -> State {
    state
}

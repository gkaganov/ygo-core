use crate::cards::CardName;
use crate::cards::CardName::{DarkHole, PotOfGreed};

mod cards;
fn main() {
    let player1 = Player::new(
        String::from("Alice"),
        vec![PotOfGreed, PotOfGreed, PotOfGreed, DarkHole],
    );
    let player2 = Player::new(String::from("Bob"), vec![PotOfGreed, DarkHole]);
    let core = Core::new([player1, player2]);
    println!("Initialized {:#?}", core)
}

#[derive(Clone, Debug)]
struct Player {
    name: String,
    deck: Vec<CardName>,
    hand: Vec<CardName>,
}
impl Player {
    fn new(name: String, deck: Vec<CardName>) -> Self {
        let (hand, deck_after_draw) = deck.split_at(1);
        Self {
            name,
            deck: deck_after_draw.to_vec(),
            hand: hand.to_vec(),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    turn_player: usize,
    players: [Player; 2],
}

#[derive(Clone, Debug)]
struct Core {
    state: State,
}
impl Core {
    pub fn new(players: [Player; 2]) -> Self {
        Core {
            state: State {
                players,
                turn_player: 0,
            },
        }
    }
}

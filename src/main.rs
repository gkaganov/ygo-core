use crate::cards::CardName::{DarkHole, PotOfGreed};
use crate::cards::{CardName, CARDS};

mod cards;
fn main() {
    let player1 = Player::new(vec![PotOfGreed, PotOfGreed, PotOfGreed, DarkHole]);
    let player2 = Player::new(vec![PotOfGreed, DarkHole]);
    let core = Core::new([player1, player2]);
    println!("Initialized {:#?}", core);
    let core_after_pot_of_greed = core.activate_card(0);
    println!("Activated Pot of Greed {:#?}", core_after_pot_of_greed);
}

#[derive(Clone, Debug)]
struct Player {
    hand: Vec<CardName>,
    deck: Vec<CardName>,
}
impl Player {
    fn new(deck: Vec<CardName>) -> Self {
        let (hand, deck_after_draw) = deck.split_at(1);
        Self {
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

    pub fn activate_card(&self, position_in_hand: usize) -> Core {
        let card_name = self
            .state
            .players
            .get(self.state.turn_player)
            .unwrap()
            .hand
            .get(position_in_hand)
            .unwrap();
        let card = CARDS.get(card_name).unwrap();
        let state = (card.effect)(self.state.clone());
        Core { state }
    }
}

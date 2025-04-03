use crate::ygo_core::private::state::State;
use crate::ygo_core::private::types::{Card, Player};

#[derive(Clone, Debug)]
pub struct Core {
    state: State,
}
impl Core {
    pub fn new(decks: [Vec<Card>; 2]) -> Self {
        Core {
            state: State {
                players: decks.map(|deck| Player::new(deck)),
                turn_player: 0,
            },
        }
    }

    pub fn activate_card(&self, position_in_hand: usize) -> Core {
        let card = self
            .state
            .players
            .get(self.state.turn_player)
            .unwrap()
            .hand
            .get(position_in_hand)
            .unwrap();
        let state = (card.effect)(self.state.clone());
        Core { state }
    }
}

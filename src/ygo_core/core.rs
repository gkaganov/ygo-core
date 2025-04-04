use crate::ygo_core::private::state::State;
use crate::ygo_core::private::types::{Card, Player};

#[derive(Clone, Debug)]
pub struct Core {
    state: State,
}
impl Core {
    pub fn new(decks: [Vec<Card>; 2]) -> Self {
        Core {
            state: State::new(decks.map(|deck| Player::new(deck))),
        }
    }

    pub fn activate_card(&self, position_in_hand: usize) -> Core {
        let card = self.state.turn_player().hand.get(position_in_hand).unwrap();
        let state = (card.effect)(self.state.clone());
        Core { state }
    }
}

use crate::ygo_core::state::{Player, PlayerAction, State};
use crate::ygo_core::types::Deck;

pub static INITIAL_HAND_SIZE: usize = 1;
pub static MIN_DECK_SIZE: usize = 1;
pub static MAX_DECK_SIZE: usize = 60;

#[derive(Clone, Debug)]
pub struct Game {
    pub state: State,
}
impl Game {
    pub fn new(decks: [Deck; 2]) -> Self {
        Game {
            state: State::new(decks.map(|deck| Player::new(deck))),
        }
    }

    pub fn legal_actions(&self) -> Vec<PlayerAction> {
        self.state.legal_actions()
    }

    pub fn take_action(&self, choice: &PlayerAction) -> Result<Game, String> {
        let source_player_index = self.state.prio_player_index;
        if !self.state.legal_actions().contains(choice) {
            Err(String::from("action not valid"))
        } else {
            Ok(match choice {
                PlayerAction::ActivateCardInHand(card_index) => Game {
                    state: self
                        .state
                        .activate_card_in_hand(source_player_index, *card_index),
                },
                PlayerAction::ActivateCardInMainMonsterZone(_) => self.clone(),
                PlayerAction::ActivateCardInExtraMonsterZone(_) => self.clone(),
                PlayerAction::ActivateCardInSpellTrapZone(_) => self.clone(),
                PlayerAction::ActivateCardInFieldSpellZone(_) => self.clone(),
                PlayerAction::ActivateCardInGraveyard(_) => self.clone(),
            })
        }
    }
}

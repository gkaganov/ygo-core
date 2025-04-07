use crate::ygo_core::card_db::Card;
use crate::ygo_core::private::types::PlayerAction::ActivateCardInHand;
use crate::ygo_core::private::types::{Player, PlayerAction};
use crate::ygo_core::types::Deck;

#[derive(Clone, Debug)]
pub struct State {
    pub players: [Player; 2],
    turn_player_index: usize,
    prio_player_index: usize,
}
impl State {
    pub fn new(players: [Player; 2]) -> Self {
        State {
            players,
            turn_player_index: 0,
            prio_player_index: 0,
        }
    }

    pub(super) fn turn_player(&self) -> &Player {
        &self.players[self.turn_player_index]
    }

    pub(super) fn prio_player(&self) -> &Player {
        &self.players[self.prio_player_index]
    }

    pub(crate) fn legal_actions(&self) -> Vec<PlayerAction> {
        let activatable_in_hand = self
            .prio_player()
            .hand
            .iter()
            .enumerate()
            .filter_map(|(index, card)| {
                Self::is_activatable(card).then(|| ActivateCardInHand(index))
            })
            .collect();
        activatable_in_hand
    }

    pub(crate) fn draw_cards(&self, amount: usize) -> State {
        let mut players = self.players.clone();
        let (hand, deck_after_draw) = players[self.prio_player_index]
            .deck
            .cards()
            .split_at(amount);
        players[self.prio_player_index]
            .hand
            .append(&mut hand.to_vec());
        players[self.prio_player_index].deck = Deck::new(deck_after_draw.to_vec()).unwrap();

        State {
            players,
            turn_player_index: self.turn_player_index,
            prio_player_index: self.prio_player_index,
        }
    }

    pub fn activate_card_in_hand(&self, position: usize) -> State {
        let card = self.prio_player().hand.get(position).unwrap();
        (card.effect)(self.clone())
    }

    fn is_activatable(card: &Card) -> bool {
        true
    }
}

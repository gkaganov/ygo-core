use crate::ygo_core::card_db::Card;
use crate::ygo_core::game::{Deck, INITIAL_HAND_SIZE};
use crate::ygo_core::state::GamePhase::DrawPhase;
use crate::ygo_core::state::PlayerAction::ActivateCardInHand;

#[derive(Clone, Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub deck: Deck,
}
impl Player {
    pub fn new(deck: Deck) -> Self {
        let (hand, deck_after_draw) = deck.cards().split_at(INITIAL_HAND_SIZE);
        Self {
            deck: Deck::new(deck_after_draw.to_vec()).unwrap(),
            hand: hand.to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    ActivateCardInHand(usize),
    ActivateCardInMainMonsterZone(usize),
    ActivateCardInExtraMonsterZone(usize),
    ActivateCardInSpellTrapZone(usize),
    ActivateCardInFieldSpellZone(usize),
    ActivateCardInGraveyard(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GamePhase {
    DrawPhase,
    StandbyPhase,
    MainPhase1,
    BattlePhase(BattlePhaseStep),
    MainPhase2,
    EndPhase,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum BattlePhaseStep {
    StartStep,
    BattleStep,
    DamageStep,
    EndStep,
}

#[derive(Clone, Debug)]
pub struct State {
    pub players: [Player; 2],
    pub turn_player_index: usize,
    pub prio_player_index: usize,
    pub current_phase: GamePhase,
}
impl State {
    pub fn new(players: [Player; 2]) -> Self {
        State {
            players,
            turn_player_index: 0,
            prio_player_index: 0,
            current_phase: DrawPhase,
        }
    }

    pub(in crate::ygo_core) fn turn_player(&self) -> &Player {
        &self.players[self.turn_player_index]
    }

    pub(in crate::ygo_core) fn prio_player(&self) -> &Player {
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

    pub(crate) fn draw_cards(&self, source_player_index: usize, amount: usize) -> State {
        let mut new_player = self.players.get(source_player_index).unwrap().clone();
        let (hand, deck_after_draw) = new_player.deck.cards().split_at(amount);
        new_player.hand.append(&mut hand.to_vec());
        new_player.deck = Deck::new(deck_after_draw.to_vec()).unwrap();

        let mut new_players = self.players.clone();
        new_players[source_player_index] = new_player;

        State {
            players: new_players,
            ..self.clone()
        }
    }

    pub fn activate_card_in_hand(&self, source_player_index: usize, position: usize) -> State {
        let source_player = self.players.get(source_player_index).unwrap().clone();
        let card = source_player.hand.get(position).unwrap().clone();
        (card.effect)(source_player_index, self.clone())
    }

    fn is_activatable(card: &Card) -> bool {
        true
    }
}

use crate::ygo_core::private::types::Player;

#[derive(Clone, Debug)]
pub struct State {
    pub players: [Player; 2],
    turn_player_index: usize,
}
impl State {
    pub fn new(players: [Player; 2]) -> Self {
        State {
            players,
            turn_player_index: 0,
        }
    }

    pub fn turn_player(&self) -> &Player {
        &self.players[self.turn_player_index]
    }

    pub fn draw_cards(&self, amount: usize) -> State {
        let mut players = self.players.clone();
        let (hand, deck_after_draw) = players[self.turn_player_index].deck.split_at(amount);
        players[self.turn_player_index]
            .hand
            .append(&mut hand.to_vec());
        players[self.turn_player_index].deck = deck_after_draw.to_vec();

        State {
            players,
            turn_player_index: self.turn_player_index,
        }
    }
}

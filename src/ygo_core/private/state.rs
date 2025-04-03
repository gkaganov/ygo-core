use crate::ygo_core::private::types::Player;

#[derive(Clone, Debug)]
pub struct State {
    pub turn_player: usize,
    pub players: [Player; 2],
}
impl State {
    pub fn draw_cards(&self, amount: usize) -> State {
        let mut players = self.players.clone();
        let (hand, deck_after_draw) = players[self.turn_player].deck.split_at(amount);
        players[self.turn_player].hand.append(&mut hand.to_vec());
        players[self.turn_player].deck = deck_after_draw.to_vec();

        State {
            players,
            turn_player: self.turn_player,
        }
    }
}

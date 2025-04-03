use crate::ygo_core::private::state::State;

pub fn pot_of_greed(state: State) -> State {
    state.draw_cards(2)
}

pub fn dark_hole(state: State) -> State {
    state
}

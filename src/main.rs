fn main() {
    let player1 = Player {
        name: "Alice".to_string(),
        deck: vec![Card {
            name: String::from("Pot of Greed"),
            effect: Effect::DRAW2,
        }],
    };
    let player2 = Player {
        name: "Bob".to_string(),
        deck: vec![Card {
            name: String::from("Pot of Greed"),
            effect: Effect::DRAW2,
        }],
    };
    let core = Core::new([player1, player2]);
    println!("Initialized {:#?}", core)
}

#[derive(Debug)]
struct Player {
    name: String,
    deck: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    name: String,
    effect: Effect,
}

#[derive(Debug)]
enum Effect {
    DRAW2,
}

#[derive(Debug)]
struct Core {
    players: [Player; 2],
}
impl Core {
    pub fn new(players: [Player; 2]) -> Self {
        Core { players }
    }
}

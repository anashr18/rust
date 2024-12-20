#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits: Vec<&str> = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let values: Vec<&str> = vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];
        let mut deck: Deck = Deck { cards: vec![] };
        for suit in suits {
            deck.cards.extend(
                values
                    .iter()
                    .map(|value: &&str| format!("{} of {}", value, suit)),
            );
        }
        return deck;
    }
}
fn main() {
    println!("Hello, world!");
    let deck = Deck::new();
    println!("Here is your deck: {:#?}", deck);
}




struct Deck{
    cards: Vec<String>,
}


fn main() {
    let deck = Deck {cards:vec![]};

    println!("Here is your Deck {}", deck);
}
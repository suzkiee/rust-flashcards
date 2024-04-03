mod card; // This brings the card module into scope.
mod turn;
use card::Card;
use turn::Turn;

fn main() {
    let mut card = Card::new();

    card.add_question("What is the tallest building?");
    card.add_answer("Eiffel Tower");
    card.add_category("City Life");

    let mut turn = Turn::new(card, "Empire State");
    let mut turn2 = Turn::new(&mutcard, "Eiffel Tower");

    turn.feedback();
    turn2.feedback();
}

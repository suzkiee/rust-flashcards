use crate::{card::ReviewState, Card}; // This brings the Card struct into scope.
pub struct Turn<'a> {
  card: &'a mut Card,
  guess: String,
}

impl<'a> Turn<'a> {
  pub fn new(card: &'a mut Card, guess: &str ) -> Turn<'a> {
    Turn {
      card, 
      guess: guess.to_string(),
    }
  }

  pub fn guess(&self) -> &str {
    &self.guess
  }

  pub fn card(&self) -> &Card {
    &self.card
  }

  fn check_guess(&self, guess: &str) -> bool {
    self.card.answer() == guess
}

  pub fn feedback(&mut self) {
    if self.check_guess(self.guess()) {
      self.card.set_state(ReviewState::Passed);
      println!("CORRECT");
  } else {
      println!("WRONG");
  }
  }
}
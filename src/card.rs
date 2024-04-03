pub enum ReviewState {
    Passed,
    NotPassed,
}
pub struct Card {
    state: ReviewState,
    question: String,
    answer: String,
    category: String,
}

impl Card {
    pub fn new() -> Card {
        Card {
            state: ReviewState::NotPassed,
            question: String::new(),
            answer: String::new(),
            category: String::new(),
        }
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn add_question(&mut self, input: &str) {
        self.question.push_str(input)
    }

    pub fn add_answer(&mut self, input: &str) {
        self.answer.push_str(input)
    }

    pub fn add_category(&mut self, input: &str) {
        self.category.push_str(input)
    }

    pub fn set_state(&mut self, state: ReviewState) {
        self.state = state
    }
}

trait State {}

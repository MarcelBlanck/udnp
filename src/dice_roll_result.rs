pub struct DiceRollResult {
    pub result: i32,
    pub error: DiceRollResultError
}

pub enum DiceRollResultError {
    None,
    Syntax
}
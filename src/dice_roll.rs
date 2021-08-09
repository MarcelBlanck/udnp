use dice_roll_result::DiceRollResult;
use dice_roll_result::DiceRollResultError;

pub struct RandomSources {
    random_i32: Box<dyn RandomSource<i32>>
}

struct RandomI32;

trait RandomSource<T> {
    fn next(&self) -> T;
}

impl RandomSource<i32> for RandomI32 {
    fn next(&self) -> i32 {
        rand::random::<i32>()
    }
}

#[must_use]
pub fn roll<S>(equation: S, random_sources: &RandomSources) -> DiceRollResult where S : Into<String>{
    print!("{:?}", equation.into());
    DiceRollResult{result: random_sources.random_i32.next(), error: DiceRollResultError::None}
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRandomI32 {
        next_value: i32
    }

    impl RandomSource<i32> for MockRandomI32 {
        fn next(&self) -> i32 {
            self.next_value
        }
    }

    #[test]
    fn test_get_any_value_from_random_source() {
        let random_source = MockRandomI32{next_value: 1};
        assert_eq!(random_source.next(), 1);
    }

    #[test]
    fn test_dummy_roll() -> Result<(), String> {
        let random_sources = RandomSources {
            random_i32: Box::new(MockRandomI32 {
                next_value: 5
            })
        };
        let roll_result = roll("", &random_sources);
        assert_eq!(roll_result.result, 5);
        match roll_result.error {
            DiceRollResultError::None => Ok(()),
            _ => Err(String::from("Unexpected Syntax Error"))
        }
    }

    #[test]
    fn test_roll_with_random_sources() -> Result<(), String> {
        let random_sources = RandomSources {
            random_i32: Box::new(RandomI32)
        };
        let roll_result = roll("", &random_sources);
        println!("Rolled {}", roll_result.result);
        match roll_result.error {
            DiceRollResultError::None => Ok(()),
            _ => Err(String::from("Unexpected Syntax Error"))
        }
    }
}

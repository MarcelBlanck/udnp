pub enum RpnElement {
    Integer(i32),
    Add,
    DiceRolls
}

#[must_use]
/// # panics
pub fn infix_string_to_rpn<S>(s: S) -> Vec<RpnElement> where S : Into<String> {
    let mut rpn: Vec<RpnElement> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut number: String = String::new();

    let mut push_number = |mut n : &str| {
        if !n.is_empty() {
            rpn.push(RpnElement::Integer(n.parse::<i32>().unwrap()));
        }
    };
    
    for c in s.into().chars() {
        if c.is_digit(10) {
            number.push(c);
        } else {
            push_number(&number);
            number.clear();
            stack.push(c);
        }
    }
    push_number(&number);
    number.clear();

    return rpn;
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_empty_string_result_is_empty_vec() {
        assert_eq!(infix_string_to_rpn("").len(), 0);
    }

    #[test]
    fn test_string_with_one_int_returns_int_element() {
        let rpn = infix_string_to_rpn("12345");
        assert_eq!(rpn.len(), 1);
        assert!(matches!(rpn.get(0).unwrap(), RpnElement::Integer(12345)));
    }
}
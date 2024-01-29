#[cfg(test)]
use gegene::r#move::Square;

#[test]
fn from_algebraic_returns_correct_square_for_valid_input() {
    let mut index = 0;
    for rank in 1..=8 {
        for file in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            let square_string = format!("{}{}", file, rank);
            let square = Square::from_algebraic(&square_string).unwrap();
            assert_eq!(square.0, index);
            index += 1;
        }
    }
}

#[test]
fn from_algebraic_returns_error_for_invalid_file() {
    let result = Square::from_algebraic("i1");
    assert!(result.is_err());
}

#[test]
fn from_algebraic_returns_error_for_invalid_rank() {
    let result = Square::from_algebraic("a9");
    assert!(result.is_err());
}

#[test]
fn from_algebraic_returns_error_for_empty_string() {
    let result = Square::from_algebraic("");
    assert!(result.is_err());
}

#[test]
fn from_algebraic_returns_error_for_single_character_string() {
    let result = Square::from_algebraic("a");
    assert!(result.is_err());
}

#[test]
fn from_algebraic_returns_error_for_to_many_character_string() {
    let result = Square::from_algebraic("a1c");
    assert!(result.is_err());
}

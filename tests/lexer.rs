extern crate bbb;

use std::rc::Rc;
use bbb::lexer::*;

#[test]
fn test_state_next_from_start() {
    let state = LexerState::from("abc");
    assert_eq!(
        state.next(),
        LexerState {
            element: Some('a'),
            position: Some(0),
            value: Some("".to_owned()),
            stream: Rc::new(vec!['a', 'b', 'c']),
        }
    );
}

#[test]
fn test_state_next_from_mid() {
    let state = LexerState::from("abc");
    assert_eq!(
        state.next().next(),
        LexerState {
            element: Some('b'),
            position: Some(1),
            value: Some("".to_owned()),
            stream: Rc::new(vec!['a', 'b', 'c']),
        }
    );
}

#[test]
fn test_state_next_at_end() {
    let state = LexerState::from("abc");
    assert_eq!(
        state.next().next().next().next(),
        LexerState {
            element: None,
            position: Some(3),
            value: Some("".to_owned()),
            stream: Rc::new(vec!['a', 'b', 'c']),
        }
    );
}

#[test]
fn test_char_lexer_match() {
    let lexer = character('a');
    let state = LexerState::from("a");
    let result = lexer.run(&state);
    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('a'),
            position: Some(0),
            value: Some("a".to_owned()),
            stream: Rc::new(vec!['a']),
        })
    );
}

#[test]
fn test_char_lexer_failure() {
    let lexer = character('a');
    let state = LexerState::from("b");
    let result = lexer.run(&state);
    assert_eq!(
        result,
        Err(LexerError {
            position: Some(0),
            string: "b".to_owned(),
        })
    )
}

#[test]
fn test_string_lexer_match() {
    let lexer = string("abc");
    let state = LexerState::from("abc");
    let result = lexer.run(&state);
    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(2),
            value: Some("abc".to_owned()),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

#[test]
fn test_string_lexer_failure() {
    let lexer = string("abc");
    let mut state = LexerState::from("babc");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Err(LexerError {
            position: Some(0),
            string: "babc".to_owned(),
        })
    )
}

#[test]
fn test_or_lexer_match_first() {
    let abc = string("abc");
    let cba = string("cba");
    let lexer = or(abc, cba);
    let state = LexerState::from("abc");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(2),
            value: Some("abc".to_owned()),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

#[test]
fn test_or_lexer_match_other() {
    let abc = string("abc");
    let cba = string("cba");
    let lexer = or(abc, cba);
    let state = LexerState::from("cba");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('a'),
            position: Some(2),
            value: Some("cba".to_owned()),
            stream: Rc::new(vec!['c', 'b', 'a']),
        })
    );
}

#[test]
fn test_or_lexer_match_compound() {
    let abc = string("abc");
    let cba = string("cba");
    let abc_or_cba = or(abc, cba);
    let abc_or_cba_or_dog = or(abc_or_cba, string("dog"));
    let state = LexerState::from("dog");
    let result = abc_or_cba_or_dog.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('g'),
            position: Some(2),
            value: Some("dog".to_owned()),
            stream: Rc::new(vec!['d', 'o', 'g'])
        })
    )
}

#[test]
fn test_map_match() {
    let abc = string("abc");
    let lexer = map(abc, |_| 100);
    let state = LexerState::from("abc");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(2),
            value: Some(100),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

#[test]
fn test_try_match() {
    let abc = string("abc");
    let lexer = try(abc);
    let state = LexerState::from("abc");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(2),
            value: Some("abc".to_owned()),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

#[test]
fn test_try_fail() {
    let lexer = try(string("abc"));
    let state = LexerState::from("babc");
    let result = lexer.run(&state);

    assert_eq!(result, Ok(state));
}

#[test]
fn test_many_match_single() {
    let lexer = many(string("abc"));
    let state = LexerState::from("abc");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(2),
            value: Some(vec!["abc".to_owned()]),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

#[test]
fn test_many_match_many() {
    let lexer = many(string("abc"));
    let state = LexerState::from("abcabc");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(5),
            value: Some(vec!["abc".to_owned(), "abc".to_owned()]),
            stream: Rc::new(vec!['a', 'b', 'c', 'a', 'b', 'c']),
        })
    );

}

#[test]
fn test_many_match_many_complex() {
    let lexer = many(or(string("abc"), string("bac")));
    let state = LexerState::from("abcbac");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some('c'),
            position: Some(5),
            value: Some(vec!["abc".to_owned(), "bac".to_owned()]),
            stream: Rc::new(vec!['a', 'b', 'c', 'b', 'a', 'c']),
        })
    );
}

#[test]
fn test_between_match() {
    let lexer = between(character('('), character(')'), string("abc"));
    let state = LexerState::from("(abc)");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some(')'),
            position: Some(4),
            value: Some("abc".to_owned()),
            stream: Rc::new(vec!['(', 'a', 'b', 'c', ')']),
        })
    );
}

#[test]
fn test_between_complex_match() {
    let lexer = between(
        character('('), character(')'),
        many(or(string("abc"), string("bac")))
    );
    let state = LexerState::from("(abcbacbacabc)");
    let result = lexer.run(&state);

    assert_eq!(
        result,
        Ok(LexerState {
            element: Some(')'),
            position: Some(13),
            value: Some(vec!["abc".to_owned(), "bac".to_owned(), "bac".to_owned(), "abc".to_owned()]),
            stream: Rc::new(vec!['(', 'a', 'b', 'c', 'b', 'a', 'c', 'b', 'a', 'c', 'a', 'b', 'c', ')']),
        })
    );
}

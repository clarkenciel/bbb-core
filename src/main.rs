mod interpreter;

use interpreter::token::Token;

fn main() {
    println!("Hello, world!");
}

type SymbolStream = Iterator<Item = char>;

struct LexerError {
    position: u32,
    symbol: String,
}

#[derive(Clone)]
struct LexerState<'a> {
    current_string: String,
    position: u32,
    stream: &'a SymbolStream,
    tokens: Vec<Token>,
}

impl<'a> LexerState<'a> {
    fn add_character(&self, character: char) -> Self {
        let mut new_state = self.clone();
        new_state.current_string.push(character);
        new_state.position += 1;
        new_state
    }

    fn add_string(&self, string: &str) -> Self {
        let mut new_state = self.clone();
        new_state.current_string.push_str(string);
        new_state.position += string.len() as u32;
        new_state
    }
}

impl<'a> From<LexerState<'a>> for LexerError {
    fn from(state: LexerState) -> Self {
        LexerError {
            position: state.position,
            symbol: state.current_string,
        }
    }
}

type LexerResult<'a> = Result<LexerState<'a>, LexerError>;

trait Lexer<'a> {
    fn run(&self, state: &'a LexerState<'a>) -> LexerResult<'a>;
}

// or /////////////////////////////////////////////////////////////////////////

struct OrLexer<'a> {
    option_a: &'a Lexer<'a>,
    option_b: &'a Lexer<'a>,
}

impl<'a> OrLexer<'a> {
    fn new(option_a: &'a Lexer<'a>, option_b: &'a Lexer<'a>) -> Self {
        OrLexer {
            option_a: option_a,
            option_b: option_b,
        }
    }
}

impl<'a> Lexer<'a> for OrLexer<'a> {
    fn run(&self, state: &'a LexerState<'a>) -> LexerResult<'a> {
        self.option_a.run(state).or_else(
            |_| self.option_b.run(state),
        )
    }
}

fn or<'a>(option_a: &'a Lexer<'a>, option_b: &'a Lexer<'a>) -> OrLexer<'a> {
    OrLexer::new(option_a, option_b)
}

// try ////////////////////////////////////////////////////////////////////////

struct TryLexer<'a> {
    lexer: &'a Lexer<'a>,
}

impl<'a> TryLexer<'a> {
    fn new(lexer: &'a Lexer<'a>) -> Self {
        TryLexer { lexer: lexer }
    }
}

impl<'a> Lexer<'a> for TryLexer<'a> {
    fn run(&self, state: &'a LexerState<'a>) -> LexerResult<'a> {
        let old_state = state.clone();
        match self.lexer.run(state) {
            Err(_) => Ok(old_state),
            result => result,
        }
    }
}

// character //////////////////////////////////////////////////////////////////
struct CharLexer {
    character: char,
}

impl CharLexer {
    fn new(character: char) -> Self {
        CharLexer { character: character }
    }
}

impl<'a> Lexer<'a> for CharLexer {
    fn run(&self, state: &'a LexerState<'a>) -> LexerResult<'a> {
        state
            .stream
            .next()
            .and_then(|c| if c == self.character {
                Some(state.add_character(c))
            } else {
                None
            })
            .ok_or(LexerError::from(*state))
    }
}

// string /////////////////////////////////////////////////////////////////////
struct StringLexer<'a> {
    string: &'a str,
}

impl<'a> StringLexer<'a> {
    fn new(string: &'a str) -> Self {
        StringLexer { string: string }
    }
}

impl<'a> From<&'a str> for StringLexer<'a> {
    fn from(string: &'a str) -> Self {
        StringLexer::new(string)
    }
}

impl<'a> From<String> for StringLexer<'a> {
    fn from(string: String) -> Self {
        StringLexer::new(string.as_str())
    }
}

impl<'a> Lexer<'a> for StringLexer<'a> {
    fn run(&self, state: &'a LexerState<'a>) -> LexerResult<'a> {
        let test_string = state.stream.take(self.string.len()).collect();
        if test_string == self.string {
            state.stream = state.stream.skip(self.string.len());
            Ok(state.add_string(self.string))
        } else {
            Err(LexerError::from(*state))
        }
    }
}

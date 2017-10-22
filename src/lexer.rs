use std::rc::Rc;

type SymbolStream = Vec<char>;


#[derive(Clone, Debug, PartialEq, Eq)]
struct LexerState<T>
where
    T: Clone,
{
    value: T,
    position: u32,
    stream: Rc<SymbolStream>,
}

impl<T> LexerState<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<char> {
        let output = self.stream[self.position as usize..self.stream.len()]
            .iter()
            .next()
            .map(|val| *val);
        self.position += 1;
        output
    }
}

impl LexerState<String> {
    fn add_character(&mut self, character: char) -> Self {
        self.value.push(character);
        self.clone()
    }

    fn add_string(&mut self, string: &str) -> Self {
        self.value.push_str(string);
        self.clone()
    }
}

impl<'a> From<&'a str> for LexerState<String> {
    fn from(string: &'a str) -> LexerState<String> {
        LexerState {
            position: 0,
            value: "".to_owned(),
            stream: Rc::new(string.chars().collect()),
        }
    }
}

// LexerError /////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
struct LexerError<T> {
    position: u32,
    symbol: T,
}

impl<'a, T> From<LexerState<T>> for LexerError<T>
where
    T: Clone,
{
    fn from(state: LexerState<T>) -> Self {
        LexerError {
            position: state.position,
            symbol: state.value,
        }
    }
}

impl<'a, T> From<&'a LexerState<T>> for LexerError<T>
where
    T: Clone,
{
    fn from(state: &LexerState<T>) -> Self {
        LexerError {
            position: state.position,
            symbol: state.value.clone(),
        }
    }
}

// lexer result ///////////////////////////////////////////////////////////////

type LexerResult<'a, I, O> = Result<LexerState<O>, LexerError<I>>;

// lexer trait ////////////////////////////////////////////////////////////////

trait Lexer<'a, I, O>
where
    I: Clone + 'a,
    O: Clone + 'a,
{
    fn run(&self, state: &mut LexerState<I>) -> LexerResult<'a, I, O>;
}

// or /////////////////////////////////////////////////////////////////////////

struct OrLexer<'a, I: 'a, O: 'a> {
    option_a: &'a Lexer<'a, I, O>,
    option_b: &'a Lexer<'a, I, O>,
}

impl<'a, I, O> OrLexer<'a, I, O> {
    fn new(option_a: &'a Lexer<'a, I, O>, option_b: &'a Lexer<'a, I, O>) -> Self {
        OrLexer {
            option_a: option_a,
            option_b: option_b,
        }
    }
}

impl<'a, I, O> Lexer<'a, I, O> for OrLexer<'a, I, O>
where
    I: Clone,
    O: Clone,
{
    fn run(&self, state: &mut LexerState<I>) -> LexerResult<'a, I, O> {
        self.option_a.run(state).or_else(
            |_| self.option_b.run(state),
        )
    }
}

fn or<'a, I, O>(a: &'a Lexer<'a, I, O>, b: &'a Lexer<'a, I, O>) -> OrLexer<'a, I, O> {
    OrLexer::new(a, b)
}


// try ////////////////////////////////////////////////////////////////////////

struct TryLexer<'a, I: 'a> {
    lexer: &'a Lexer<'a, I, I>,
}

impl<'a, I> TryLexer<'a, I> {
    fn new(lexer: &'a Lexer<'a, I, I>) -> Self {
        TryLexer { lexer: lexer }
    }
}

impl<'a, I> Lexer<'a, I, I> for TryLexer<'a, I>
where
    I: Clone,
{
    fn run(&self, state: &mut LexerState<I>) -> LexerResult<'a, I, I> {
        match self.lexer.run(state) {
            Err(_) => Ok(state.clone()),
            result => result,
        }
    }
}

fn try<'a, I>(lexer: &'a Lexer<'a, I, I>) -> TryLexer<'a, I> {
    TryLexer::new(lexer)
}

// map ////////////////////////////////////////////////////////////////////////

struct MapLexer<'a, I: 'a, O: 'a> {
    function: &'a Fn(I) -> O,
    lexer: &'a Lexer<'a, I, I>,
}

impl<'a, I, O> MapLexer<'a, I, O>
where
    I: Clone + 'a,
    O: Clone + 'a,
{
    fn new(lexer: &'a Lexer<'a, I, I>, f: &'a Fn(I) -> O) -> MapLexer<'a, I, O> {
        MapLexer {
            function: f,
            lexer: lexer,
        }
    }
}

impl<'a, I, O> Lexer<'a, I, O> for MapLexer<'a, I, O>
where
    I: Clone + 'a,
    O: Clone + 'a,
{
    fn run(&self, state: &mut LexerState<I>) -> LexerResult<'a, I, O> {
        let transform = self.function;
        self.lexer.run(state).map(|ref new_state| LexerState {
            value: transform(new_state.clone().value),
            position: new_state.position,
            stream: new_state.stream.clone(),
        })
    }
}

fn map<'a, I, O>(lexer: &'a Lexer<'a, I, I>, f: &'a Fn(I) -> O) -> MapLexer<'a, I, O>
where
    I: Clone + 'a,
    O: Clone + 'a,
{
    MapLexer::new(lexer, f)
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

impl<'a> Lexer<'a, String, String> for CharLexer {
    fn run(&self, state: &mut LexerState<String>) -> LexerResult<'a, String, String> {
        state
            .next()
            .and_then(|c| if c == self.character {
                Some(state.add_character(c))
            } else {
                None
            })
            .ok_or(LexerError::from(&*state))
    }
}

// string /////////////////////////////////////////////////////////////////////

struct StringLexer {
    string: String,
    char_lexers: Vec<CharLexer>,
}

impl StringLexer {
    fn new(string: &str) -> Self {
        StringLexer {
            char_lexers: string.chars().map(|c| CharLexer::new(c)).collect(),
            string: string.to_owned(),
        }
    }
}

impl<'a> From<&'a str> for StringLexer {
    fn from(string: &'a str) -> Self {
        StringLexer::new(string)
    }
}

impl From<String> for StringLexer {
    fn from(string: String) -> Self {
        StringLexer::new(&string)
    }
}

impl<'a> Lexer<'a, String, String> for StringLexer {
    fn run(&self, state: &mut LexerState<String>) -> LexerResult<'a, String, String> {
        self.char_lexers.iter().fold(
            Ok(state.clone()),
            |running, lexer| {
                running.and_then(|ref mut new_state| lexer.run(new_state))
            },
        )
    }
}

// tests //////////////////////////////////////////////////////////////////////

#[test]
fn test_char_lexer_match() {
    let lexer = CharLexer::new('a');
    let mut state = LexerState::from("a");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Ok(LexerState {
            position: 1,
            value: "a".to_owned(),
            stream: Rc::new(vec!['a']),
        })
    );
}

#[test]
fn test_char_lexer_failure() {
    let lexer = CharLexer::new('a');
    let mut state = LexerState::from("b");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Err(LexerError {
            position: 1,
            symbol: "".to_owned(),
        })
    )
}

#[test]
fn test_string_lexer_match() {
    let lexer = StringLexer::from("abc");
    let mut state = LexerState::from("abc");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Ok(LexerState {
            position: 3,
            value: "abc".to_owned(),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

#[test]
fn test_string_lexer_failure() {
    let lexer = StringLexer::from("abc");
    let mut state = LexerState::from("babc");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Err(LexerError {
            position: 1,
            symbol: "".to_owned(),
        })
    )
}

#[test]
fn test_or_lexer_match() {
    let abc = StringLexer::from("abc");
    let cba = StringLexer::from("cba");
    let lexer = or(&abc, &cba);

    let mut state = LexerState::from("abc");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Ok(LexerState {
            position: 3,
            value: "abc".to_owned(),
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );

    let mut state = LexerState::from("cba");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Ok(LexerState {
            position: 3,
            value: "cba".to_owned(),
            stream: Rc::new(vec!['c', 'b', 'a']),
        })
    );
}

#[test]
fn test_map_match() {
    let abc = StringLexer::from("abc");
    let lexer = map(&abc, &|_| 100);

    let mut state = LexerState::from("abc");
    let result = lexer.run(&mut state);
    assert_eq!(
        result,
        Ok(LexerState {
            position: 3,
            value: 100,
            stream: Rc::new(vec!['a', 'b', 'c']),
        })
    );
}

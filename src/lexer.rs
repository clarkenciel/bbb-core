use std::rc::Rc;

type SymbolStream = Vec<char>;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexerState<T>
where
    T: Clone,
{
    value: Option<T>,
    position: Option<usize>,
    element: Option<char>,
    stream: Rc<SymbolStream>,
}

impl<T> LexerState<T>
where
    T: Clone,
{
    pub fn next(&self) -> LexerState<T> {
        let position = self.position.map(|position| position + 1).or(Some(0));
        let element = position.and_then(|position| self.stream.get(position).map(|&x| x));

        LexerState {
            element: element,
            position: position,
            stream: self.stream.clone(),
            value: self.value.clone(),
        }
    }

    fn is_done(&self) -> bool {
        Some(self.stream.len()) <= self.position
    }
}

impl LexerState<String> {
    fn add_character(&self, character: char) -> Self {
        let mut output = self.clone();
        output.value = output
            .value
            .map(|mut val| {
                val.push(character);
                val
            })
            .or(Some(character.to_string()));
        output
    }

    fn add_string(&self, string: &str) -> Self {
        let mut output = self.clone();
        output.value = output
            .value
            .map(|mut val| {
                val.push_str(string);
                val
            })
            .or(Some(string.to_owned()));
        output
    }
}

impl<'a> From<&'a str> for LexerState<String> {
    fn from(string: &'a str) -> LexerState<String> {
        LexerState {
            element: None,
            position: None,
            value: Some("".to_owned()),
            stream: Rc::new(string.chars().collect()),
        }
    }
}

impl<'a, T: Clone + 'a> From<&'a LexerState<T>> for LexerState<T> {
    fn from(other: &'a LexerState<T>) -> Self {
        other.clone()
    }
}

// LexerError /////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub struct LexerError {
    position: Option<usize>,
    string: String,
}

impl<'a, T> From<LexerState<T>> for LexerError
where
    T: Clone + 'a,
{
    fn from(state: LexerState<T>) -> Self {
        LexerError {
            position: state.position,
            string: state.stream.iter().collect(),
        }
    }
}

impl<'a, T> From<&'a LexerState<T>> for LexerError
where
    T: Clone + 'a,
{
    fn from(state: &LexerState<T>) -> Self {
        LexerError {
            position: state.position,
            string: state.stream.iter().collect(),
        }
    }
}

// lexer result ///////////////////////////////////////////////////////////////

pub type LexerResult<'a, O> = Result<LexerState<O>, LexerError>;

// lexer trait ////////////////////////////////////////////////////////////////

pub trait Lexer<'a, I, O>
where
    I: Clone + 'a,
    O: Clone + 'a,
{
    fn run(&self, state: &LexerState<I>) -> LexerResult<'a, O>;
}

type BasicLexer<'a, T> = &'a Lexer<'a, T, T>;
type TransformLexer<'a, I, O> = &'a Lexer<'a, I, O>;

// or /////////////////////////////////////////////////////////////////////////

pub struct OrLexer<'a, I: 'a, O: 'a> {
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
    fn run(&self, state: &LexerState<I>) -> LexerResult<'a, O> {
        self.option_a.run(state).or_else(
            |_| self.option_b.run(state),
        )
    }
}

pub fn or<'a, I, O>(a: &'a Lexer<'a, I, O>, b: &'a Lexer<'a, I, O>) -> OrLexer<'a, I, O> {
    OrLexer::new(a, b)
}


// try ////////////////////////////////////////////////////////////////////////

pub struct TryLexer<'a, I: 'a> {
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
    fn run(&self, state: &LexerState<I>) -> LexerResult<'a, I> {
        match self.lexer.run(state) {
            Err(_) => Ok(state.clone()),
            result => result,
        }
    }
}

pub fn try<'a, I>(lexer: &'a Lexer<'a, I, I>) -> TryLexer<'a, I> {
    TryLexer::new(lexer)
}

// map ////////////////////////////////////////////////////////////////////////

pub struct MapLexer<'a, I: 'a, O: 'a> {
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
    fn run(&self, state: &LexerState<I>) -> LexerResult<'a, O> {
        let transform = self.function;
        self.lexer.run(state).map(|ref new_state| {
            LexerState {
                element: new_state.element,
                value: new_state.clone().value.map(transform),
                position: new_state.position,
                stream: new_state.stream.clone(),
            }
        })
    }
}

pub fn map<'a, I, O>(lexer: &'a Lexer<'a, I, I>, f: &'a Fn(I) -> O) -> MapLexer<'a, I, O>
where
    I: Clone + 'a,
    O: Clone + 'a,
{
    MapLexer::new(lexer, f)
}

// many ///////////////////////////////////////////////////////////////////////

pub struct ManyLexer<'a, I: 'a> {
    lexer: &'a Lexer<'a, I, I>,
}

impl<'a, I: 'a> ManyLexer<'a, I> {
    fn new(lexer: &'a Lexer<'a, I, I>) -> Self {
        ManyLexer { lexer: lexer }
    }
}

impl<'a, I: 'a> From<&'a Lexer<'a, I, I>> for ManyLexer<'a, I> {
    fn from(lexer: &'a Lexer<'a, I, I>) -> Self {
        ManyLexer::new(lexer)
    }
}

use std::fmt::Debug;
impl<'a, I: 'a + Clone + Debug> Lexer<'a, I, Vec<I>> for ManyLexer<'a, I> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<'a, Vec<I>> {
        let lexer = try(self.lexer);
        let mut next_state = state.clone();
        let mut output = LexerState {
            element: state.element,
            value: Some(vec![]),
            position: state.position,
            stream: state.stream.clone(),
        };

        loop {
            match lexer.run(&next_state) {
                Ok(mut new_state) => {
                    if new_state.position > output.position {
                        if let Some(value) = new_state.value {
                            output.value = output.value.map(|mut vec| {
                                vec.push(value.clone());
                                vec
                            });
                        }
                        output.element = new_state.element;
                        output.position = new_state.position;
                        new_state.value = None;
                        next_state = new_state;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(output)
    }
}

pub fn many<'a, I>(lexer: &'a Lexer<'a, I, I>) -> ManyLexer<'a, I> {
    ManyLexer::new(lexer)
}

// character //////////////////////////////////////////////////////////////////

pub struct CharLexer {
    character: char,
}

impl CharLexer {
    fn new(character: char) -> Self {
        CharLexer { character: character }
    }
}

impl<'a> Lexer<'a, String, String> for CharLexer {
    fn run(&self, state: &LexerState<String>) -> LexerResult<'a, String> {
        let next_state = state.next();
        next_state
            .element
            .and_then(|c| if c == self.character {
                Some(next_state.add_character(c))
            } else {
                None
            })
            .ok_or(LexerError::from(&next_state))
    }
}

pub fn character<'a>(character: char) -> CharLexer {
    CharLexer::new(character)
}

// string /////////////////////////////////////////////////////////////////////

pub struct StringLexer {
    char_lexers: Vec<CharLexer>,
}

impl StringLexer {
    fn new(string: &str) -> Self {
        StringLexer { char_lexers: string.chars().map(|c| CharLexer::new(c)).collect() }
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
    fn run(&self, state: &LexerState<String>) -> LexerResult<'a, String> {
        self.char_lexers.iter().fold(
            Ok(state.clone()),
            |running, lexer| {
                running.and_then(|ref new_state| lexer.run(new_state))
            },
        )
    }
}

pub fn string<'a>(string: &'a str) -> StringLexer {
    StringLexer::from(string)
}

// between ////////////////////////////////////////////////////////////////////

pub struct BetweenLexer<'a, I: 'a + Clone, O: 'a + Clone> {
    open_lexer: BasicLexer<'a, I>,
    close_lexer: BasicLexer<'a, I>,
    body_lexer: TransformLexer<'a, I, O>,
}

impl<'a, I: 'a + Clone, O: 'a + Clone> BetweenLexer<'a, I, O> {
    fn new(
        open: BasicLexer<'a, I>,
        close: BasicLexer<'a, I>,
        body: TransformLexer<'a, I, O>,
    ) -> BetweenLexer<'a, I, O> {
        BetweenLexer {
            open_lexer: open,
            close_lexer: close,
            body_lexer: body,
        }
    }
}

pub fn between<'a, I: 'a + Clone, O: 'a + Clone>(
    open: BasicLexer<'a, I>,
    close: BasicLexer<'a, I>,
    body: TransformLexer<'a, I, O>,
) -> BetweenLexer<'a, I, O> {
    BetweenLexer::new(open, close, body)
}

impl<'a, I: 'a + Clone, O: 'a + Clone> Lexer<'a, I, O> for BetweenLexer<'a, I, O> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<'a, O> {
        self.open_lexer.run(state)
            .and_then(|state| self.body_lexer.run(&state))
            .and_then(|state| self.close_lexer.run(&state).and_then(|_| Ok(state)))
    }
}

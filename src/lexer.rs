use std::marker::PhantomData;
use std::rc::Rc;

type SymbolStream = Vec<char>;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexerState<T>
where
    T: Clone,
{
    pub value: Option<T>,
    pub position: Option<usize>,
    pub element: Option<char>,
    pub stream: Rc<SymbolStream>,
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

    fn drop_value<T2: Clone>(&self) -> LexerState<T2> {
        LexerState {
            element: self.element,
            position: self.position,
            stream: self.stream.clone(),
            value: None,
        }
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
    pub position: Option<usize>,
    pub string: String,
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

pub type LexerResult<O> = Result<LexerState<O>, LexerError>;

// lexer trait ////////////////////////////////////////////////////////////////

pub trait Lexer<I: Clone, O: Clone> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<O>;
}

// or /////////////////////////////////////////////////////////////////////////

pub struct OrLexer<I: Clone, O: Clone, A: Lexer<I, O>, B: Lexer<I, O>> {
    option_a: A,
    option_b: B,
    phantom_input: PhantomData<I>,
    phantom_output: PhantomData<O>,
}

impl<I: Clone, O: Clone, A: Lexer<I, O>, B: Lexer<I, O>> OrLexer<I, O, A, B> {
    fn new(option_a: A, option_b: B) -> Self {
        OrLexer {
            option_a: option_a,
            option_b: option_b,
            phantom_input: PhantomData,
            phantom_output: PhantomData,
        }
    }
}

impl<I: Clone, O: Clone, A: Lexer<I, O>, B: Lexer<I, O>> Lexer<I, O> for OrLexer<I, O, A, B> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<O> {
        self.option_a.run(state).or_else(
            |_| self.option_b.run(state),
        )
    }
}

pub fn or<I: Clone, O: Clone, A: Lexer<I, O>, B: Lexer<I, O>>(a: A, b: B) -> OrLexer<I, O, A, B> {
    OrLexer::new(a, b)
}


// try ////////////////////////////////////////////////////////////////////////

pub struct TryLexer<I: Clone, L: Lexer<I, I>> {
    lexer: L,
    phantom_input: PhantomData<I>,
}

impl<I: Clone, L: Lexer<I, I>> TryLexer<I, L> {
    fn new(lexer: L) -> Self {
        TryLexer {
            lexer: lexer,
            phantom_input: PhantomData,
        }
    }
}

impl<I: Clone, L: Lexer<I, I>> Lexer<I, I> for TryLexer<I, L> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<I> {
        match self.lexer.run(state) {
            Err(_) => Ok(state.clone()),
            result => result,
        }
    }
}

pub fn try<I: Clone, L: Lexer<I, I>>(lexer: L) -> TryLexer<I, L> {
    TryLexer::new(lexer)
}

// map ////////////////////////////////////////////////////////////////////////

pub struct MapLexer<I: Clone, O: Clone, F: Fn(I) -> O, L: Lexer<I, I>> {
    function: F,
    lexer: L,
    phantom_input: PhantomData<I>,
    phantom_output: PhantomData<O>,
}

impl<I: Clone, O: Clone, F: Fn(I) -> O, L: Lexer<I, I>> MapLexer<I, O, F, L> {
    fn new(lexer: L, f: F) -> MapLexer<I, O, F, L> {
        MapLexer {
            function: f,
            lexer: lexer,
            phantom_input: PhantomData,
            phantom_output: PhantomData,
        }
    }
}

impl<I: Clone, O: Clone, F: Fn(I) -> O, L: Lexer<I, I>> Lexer<I, O> for MapLexer<I, O, F, L> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<O> {
        self.lexer.run(state).map(|ref new_state| {
            LexerState {
                element: new_state.element,
                value: new_state.clone().value.map(&self.function),
                position: new_state.position,
                stream: new_state.stream.clone(),
            }
        })
    }
}

pub fn map<I, O, F, L>(lexer: L, f: F) -> MapLexer<I, O, F, L>
where
    I: Clone,
    O: Clone,
    F: Fn(I) -> O,
    L: Lexer<I, I>,
{
    MapLexer::new(lexer, f)
}

// many ///////////////////////////////////////////////////////////////////////

pub struct ManyLexer<I: Clone, L: Lexer<I, I>> {
    lexer: L,
    phantom_input: PhantomData<I>,
}

impl<I: Clone, L: Lexer<I, I>> ManyLexer<I, L> {
    fn new(lexer: L) -> Self {
        ManyLexer {
            lexer: lexer,
            phantom_input: PhantomData,
        }
    }
}

impl<I: Clone, L: Lexer<I, I>> From<L> for ManyLexer<I, L> {
    fn from(lexer: L) -> Self {
        ManyLexer::new(lexer)
    }
}

impl<I: Clone, L: Lexer<I, I>> Lexer<I, Vec<I>> for ManyLexer<I, L> {
    fn run(&self, state: &LexerState<I>) -> LexerResult<Vec<I>> {
        let mut next_state = state.clone();
        let mut output = LexerState {
            element: state.element,
            value: Some(vec![]),
            position: state.position,
            stream: state.stream.clone(),
        };

        loop {
            match self.lexer.run(&next_state) {
                Err(_) => break,
                Ok(mut new_state) => {
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
                }
            }
        }

        Ok(output)
    }
}

pub fn many<I: Clone, L: Lexer<I, I>>(lexer: L) -> ManyLexer<I, L> {
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

impl Lexer<String, String> for CharLexer {
    fn run(&self, state: &LexerState<String>) -> LexerResult<String> {
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

pub fn character(character: char) -> CharLexer {
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

impl Lexer<String, String> for StringLexer {
    fn run(&self, state: &LexerState<String>) -> LexerResult<String> {
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


pub struct BetweenLexer<I, O, Open, Close, Body>
where
    I: Clone,
    O: Clone,
    Open: Lexer<I, I>,
    Close: Lexer<I, I>,
    Body: Lexer<I, O>,
{
    open_lexer: Open,
    close_lexer: Close,
    body_lexer: Body,
    phantom_input: PhantomData<I>,
    phantom_output: PhantomData<O>,
}

impl<I, O, Open, Close, Body> BetweenLexer<I, O, Open, Close, Body>
where
    I: Clone,
    O: Clone,
    Open: Lexer<I, I>,
    Close: Lexer<I, I>,
    Body: Lexer<I, O>,
{
    fn new(open: Open, close: Close, body: Body) -> BetweenLexer<I, O, Open, Close, Body> {
        BetweenLexer {
            open_lexer: open,
            close_lexer: close,
            body_lexer: body,
            phantom_input: PhantomData,
            phantom_output: PhantomData,
        }
    }
}


pub fn between<I, O, Open, Close, Body>(
    open: Open,
    close: Close,
    body: Body,
) -> BetweenLexer<I, O, Open, Close, Body>
where
    I: Clone,
    O: Clone,
    Open: Lexer<I, I>,
    Close: Lexer<I, I>,
    Body: Lexer<I, O>,
{
    BetweenLexer::new(open, close, body)
}

impl<I, O, Open, Close, Body> Lexer<I, O> for BetweenLexer<I, O, Open, Close, Body>
where
    I: Clone,
    O: Clone,
    Open: Lexer<I, I>,
    Close: Lexer<I, I>,
    Body: Lexer<I, O>,
{
    fn run(&self, state: &LexerState<I>) -> LexerResult<O> {
        self.open_lexer
            .run(state)
            .and_then(|body_start| {
                self.body_lexer
                    .run(&body_start.drop_value())
                    .and_then(move |body_end| {
                        self.close_lexer
                            .run(&body_end.drop_value())
                            .map(move |final_state|
                                LexerState {
                                    element: final_state.element,
                                    position: final_state.position,
                                    stream: final_state.stream,
                                    value: body_end.value,
                                }
                            )
                    })
            })
    }
}

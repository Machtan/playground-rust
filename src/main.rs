

use std::str::CharIndices;
use std::iter::Peekable;
use std::marker::PhantomData;

pub struct Token<K> {
    pub kind: K,
    pub start: usize,
    pub end: usize,
}
impl<K> Token<K> {
    pub fn new(start: usize, end: usize, kind: K) -> Token<K> {
        Token { start, end, kind }
    }
}

pub struct Error<K> {
    pub kind: K,
    pub start: usize,
    pub pos: usize,
    pub text: String,
}
impl<K> Error<K> {
    pub fn new(kind: K, start: usize, pos: usize, text: String) -> Error<K> {
        Error { kind, start, pos, text }
    }
}

/// A general lexer.
#[derive(Debug)]
pub struct SimpleLexer<'a, T, E> {
    text: &'a str,
    pub pos: usize,
    pub chars: Peekable<CharIndices<'a>>,
    pub finished: bool,
    _t: PhantomData<T>,
    _e: PhantomData<E>,
}

impl<'a, T, E> SimpleLexer<'a, T, E> {
    pub fn new(text: &'a str) -> SimpleLexer<'a, T, E> {
        SimpleLexer {
            text: text,
            pos: 0,
            chars: text.char_indices().peekable(),
            finished: false,
            _t: PhantomData,
            _e: PhantomData,
        }
    }
}

pub trait Lex<'src> {
    type TokenKind;
    type ErrKind;
    
    /// Returns the byte index at the end of the current input character.
    #[inline]
    fn end(&mut self) -> usize;
    
    /// Returns a token of the given kind and updates the lexer's position.
    #[inline]
    fn send_token<IT: Into<Self::TokenKind>>(&mut self, kind: IT) 
        -> Result<Token<Self::TokenKind>, Error<Self::ErrKind>>;
    
    /// Returns the remainder of the text being parsed.
    #[inline]
    fn rem(&self) -> &'src str;
    
    /// Advances the lexer by a character
    #[inline]
    fn advance(&mut self) -> Result<(usize, char), String>;

    /// Checks for a char, eats it if possible then sends either the first
    /// or the second given token.
    #[inline]
    fn send_if_next<IT: Into<Self::TokenKind>, ITT: Into<Self::TokenKind>>(&mut self,
                                                            pat: &str,
                                                            if_next: IT,
                                                            if_not: ITT)
                                                            -> Result<Token<Self::TokenKind>, Error<Self::ErrKind>>;
    
    /// Raises a lexer error with the given kind.
    #[inline]
    fn error<IT>(&mut self, pos: usize, kind: Self::ErrKind) -> Result<IT, Error<Self::ErrKind>>;
    
    /// Returns whether the next character is the given.
    #[inline]
    fn peek_is(&mut self, ch: char) -> bool;
    
    /// Peeks at the next character and its index in the lexer.
    #[inline]
    fn peek(&mut self) -> Option<&(usize, char)>;
    
    /// Returns the text being lexed.
    #[inline]
    fn text(&self) -> &'src str;
    
    /// Returns whether the lexer has another token.
    #[inline]
    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }
}

impl<'src, T, E> Lex<'src> for SimpleLexer<'src, T, E> {
    type TokenKind = T;
    type ErrKind = E;
    

    #[inline]
    fn end(&mut self) -> usize {
        if let Some(&(i, _)) = self.chars.peek() {
            i
        } else {
            self.text.len()
        }
    }
    
    #[inline]
    fn text(&self) -> &'src str {
        &self.text
    }

    #[inline]
    fn send_token<IT: Into<Self::TokenKind>>(&mut self, data: IT) -> Result<Token<Self::TokenKind>, Error<Self::ErrKind>> {
        let start = self.pos;
        let end = self.end();
        self.pos = end;
        Ok(Token::new(start, end, data.into()))
    }
    
    /// Returns the remainder.
    #[inline]
    fn rem(&self) -> &'src str {
        &self.text[self.pos..]
    }
    
    #[inline]
    fn advance(&mut self) -> Result<(usize, char), String> {
        if let Some((i, ch)) = self.chars.next() {
            self.pos = i;
            Ok((i, ch))
        } else {
            Err(String::from("Unexpected EOF"))
        }
    }

    /// Checks for a char, eats it if possible then sends either the first
    /// or the second given token.
    #[inline]
    fn send_if_next<IT: Into<Self::TokenKind>, ITT: Into<Self::TokenKind>>(&mut self,
                                                            pat: &str,
                                                            if_next: IT,
                                                            if_not: ITT)
                                                            -> Result<Token<Self::TokenKind>, Error<Self::ErrKind>> {
        if self.rem().starts_with(pat) {
            for _ in pat.chars() {
                self.advance().unwrap();
            }
            self.send_token(if_next.into())
        } else {
            self.send_token(if_not.into())
        }
    }
    
    #[inline]
    /// Raises a lexer error with the given kind.
    fn error<IT>(&mut self, pos: usize, kind: Self::ErrKind) -> Result<IT, Error<Self::ErrKind>> {
        self.finished = true;
        Err(Error::new(kind, self.pos, pos, String::from(self.text)))
    }

    /// Returns whether the next character is the given.
    #[inline]
    fn peek_is(&mut self, ch: char) -> bool {
        if let Some(&(_, c)) = self.chars.peek() {
            c == ch
        } else {
            false
        }
    }
    
    #[inline]
    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }
}

macro_rules! forward_lexing {
    (
    type TokenKind = $tokenkind:ty;
    type ErrKind = $errkind:ty;
    forward $struct:ty => self.$member:ident;
    ) => {
        impl<'src> Lex<'src> for $struct {
            type TokenKind = $tokenkind;
            type ErrKind = $errkind;
    
            #[inline]
            fn end(&mut self) -> usize {
                self.$member.end()
            }
    
            #[inline]
            fn send_token<IT: Into<Self::TokenKind>>(&mut self, kind: IT) 
                -> Result<Token<Self::TokenKind>, Error<Self::ErrKind>> {
                self.$member.send_token(kind)
            }
    
            #[inline]
            fn rem(&self) -> &'src str {
                self.$member.rem()
            }
    
            #[inline]
            fn advance(&mut self) -> Result<(usize, char), String> {
                self.$member.advance()
            }

            #[inline]
            fn send_if_next<IT: Into<Self::TokenKind>, ITT: Into<Self::TokenKind>>(&mut self,
                                                                    pat: &str,
                                                                    if_next: IT,
                                                                    if_not: ITT)
                                                                    -> Result<Token<Self::TokenKind>, Error<Self::ErrKind>> {
                self.$member.send_if_next(pat, if_next, if_not)                                    
            }
    
            #[inline]
            fn error<IT>(&mut self, pos: usize, kind: Self::ErrKind) -> Result<IT, Error<Self::ErrKind>> {
                self.$member.error(pos, kind)
            }
    
            #[inline]
            fn peek_is(&mut self, ch: char) -> bool {
                self.$member.peek_is(ch)
            }
            
            #[inline]
            fn peek(&mut self) -> Option<&(usize, char)> {
                self.$member.peek()
            }
            
            #[inline]
            fn text(&self) -> &'src str {
                self.$member.text()
            }
        }
    }
}

pub struct Lexer<'src> {
    inner: SimpleLexer<'src, char, String>,
}
impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Lexer<'src> {
        Lexer {
            inner: SimpleLexer::new(text)
        }
    }
}
forward_lexing! {
    type TokenKind = char;
    type ErrKind = String;
    forward Lexer<'src> => self.inner;
}

fn main() {
    println!("lexing, woo!");
    let mut lexer = Lexer::new("Hello world what's up?");
    while lexer.has_next() {
        let (i, ch) = lexer.advance().unwrap();
        println!("{}: {}", i, ch);
    }
}

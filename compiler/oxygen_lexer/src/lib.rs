#![deny(rust_2018_idioms)]

use std::str::Chars;

// use oxygen_error::Result;
use BinaryOperation::*;
use TokenKind::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token<'src> {
    pub string: &'src str,
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize
}

impl<'src> Token<'src> {
    fn new(string: &'src str, kind: TokenKind, line: usize, column: usize) -> Token<'src> {
        Token {
            string,
            kind,
            line,
            column
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum BinaryOperation {
    Minus,
    Plus,
    Slash,
    Star,
    And,
    Or
}

#[derive(PartialEq, Debug)]
pub enum Base {
    // 0b
    Binary = 2,
    // 0o
    Octal = 8,
    // No prefix or 0d
    Decimal = 10,
    // 0x
    Hexadecimal = 16
}

#[derive(PartialEq, Debug)]
pub enum LiteralKind {
    Int { base: Base, empty_int: bool },
    Float { base: Base, empty_exponent: bool },

    // bool = terminated
    Str(bool)
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Eof,

    Comment,

    Identifier,
    Literal{ kind: LiteralKind, suffix_start: usize },

    Greater,
    GreaterEq,
    Less,
    LessEq,
    NotEq,
    Eq,
    EqEq,
    AndAnd,
    OrOr,

    // +, /, &
    BinOp(BinaryOperation),
    // +=, /=, &=
    BinOpEq(BinaryOperation),

    Semicolon,
    Bang,
    Tilde,
    Comma,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket,


    Unknown
}

impl TokenKind {
    // Returns the two tokens that make up this one, if it is made of
    // two, else None
    //
    // e.g. "&=".split_two() -> ("&", "=")
    #[allow(dead_code)]
    fn split_two(&self) -> Option<(TokenKind, TokenKind)> {
        match self {
            GreaterEq => Some((Greater, Eq)),
            EqEq => Some((Eq, Eq)),
            LessEq => Some((Less, Eq)),
            AndAnd => Some((BinOp(And), BinOp(And))),
            OrOr => Some((BinOp(Or), BinOp(Or))),
            
            BinOpEq(Plus) => Some((BinOp(Plus), Eq)),
            BinOpEq(Minus) => Some((BinOp(Minus), Eq)),
            BinOpEq(Slash) => Some((BinOp(Slash), Eq)),
            BinOpEq(Star) => Some((BinOp(Star), Eq)),
            BinOpEq(And) => Some((BinOp(And), Eq)),
            BinOpEq(Or) => Some((BinOp(Or), Eq)),

            _ => None
        }
    }
}

#[allow(dead_code)]
pub struct TokenStream<'src> {
    src: &'src str,
    chars: Chars<'src>,
    line: usize,
    column: usize,
    idx: usize
}

impl<'src> TokenStream<'src> {
    fn new(input: &'src str) -> Self {
        TokenStream {
            src: input,
            chars: input.chars(),
            line: 0,
            column: 0,
            idx: 0
        }
    }
    
    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or('\0') 
    }

    #[allow(dead_code)]
    fn peek_second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or('\0')
    }

    #[inline(always)]
    fn at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.chars.next()?;
        self.idx += 1;

        match char {
            '\n' => {
                self.line += 1;
                self.column = 0;
            },
            _ => {
                self.column += 1;
            }
        }

        Some(char)
    }

    fn advance_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.at_end() {
            self.advance();
        }
    }
    
    fn skip_whitespace(&mut self) {
        self.advance_while(|c| c.is_whitespace());
    }

    fn consume_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek() {
                '0'..='9' => {
                    has_digits = true;
                    self.advance();
                },
                _ => break
            }
        }
        has_digits
    }

    fn consume_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.advance();
                },
                _ => break
            }
        }
        has_digits
    }

    fn consume_literal_suffix(&mut self) {
        if !is_ident_start(self.peek()) { return };

        self.advance();
        self.advance_while(is_ident_continue);
    }

    fn consume_float_exponent(&mut self) -> bool {
        match self.peek() {
            '+' | '-' => { self.advance(); },
            _ => {}
        };
        self.consume_decimal_digits()
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;
        if first_digit == '0' {
            let has_digits = match self.peek() {
                'b' => {
                    base = Base::Binary;
                    self.advance();
                    self.consume_decimal_digits()
                },
                'o' => {
                    base = Base::Octal;
                    self.advance();
                    self.consume_decimal_digits()
                },
                'd' => {
                    self.advance();
                    self.consume_decimal_digits()
                },
                'x' => {
                    base = Base::Hexadecimal;
                    self.advance();
                    self.consume_hexadecimal_digits()
                },
                '0'..='9' | '.' | 'e' | 'E' => {
                    self.consume_decimal_digits();
                    true
                },
                // Just a 0
                _ => return LiteralKind::Int { base, empty_int: false }
            };

            // Base prefix provided, but no digits
            // e.g. "0x"
            if !has_digits {
                return LiteralKind::Int { base, empty_int: true };
            }
        } else {
            self.consume_decimal_digits();
        }

        match self.peek() {
            // Need to check second, in case this is a field access
            // on an integer literal
            //
            // e.g. 13.as_string();
            '.' if !is_ident_start(self.peek_second()) => {
                self.advance();
                let mut empty_exponent = false;
                if self.peek().is_digit(10) {
                    self.consume_decimal_digits();
                    match self.peek() {
                        'e' | 'E' => {
                            self.advance();
                            empty_exponent = !self.consume_float_exponent();
                        }
                        _ => {}
                    }
                }
                LiteralKind::Float { base, empty_exponent }
            }
            'e' | 'E' => {
                self.advance();
                let empty_exponent = !self.consume_float_exponent();
                LiteralKind::Float { base, empty_exponent }
            }
            _ => LiteralKind::Int { base, empty_int: false }
        }
    }

    // Returns true if string is terminated
    fn double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.advance() {
            match c {
                '"' => return true,
                '\\' if self.peek() == '\\' || self.peek() == '"' => {
                    // Handle escaped \ and "
                    self.advance();
                },
                _ => {}
            };
        }

        // EOF reached
        false
    }

    fn next_token(&mut self) -> Token<'src> {
        self.skip_whitespace();

        let start_idx = self.idx;
        let line = self.line;
        let column = self.column;

        let c = match self.advance() {
            Some(c) => c,
            None => return Token::new("", TokenKind::Eof, line, column)
        };

        let kind = match c {
            '#' => {
                self.advance_while(|c| c != '\n');
                Comment
            },

            c if is_ident_start(c) => {
                self.advance_while(is_ident_continue);
                Identifier
            },

            c @ '0'..='9' => {
                let literal_kind = self.number(c);
                let suffix_start = self.idx - start_idx;
                self.consume_literal_suffix();
                TokenKind::Literal { kind: literal_kind, suffix_start }
            }, 

            '&' => {
                match self.peek() {
                    '&' => {
                        self.advance();
                        AndAnd
                    },
                    '=' => {
                        self.advance();
                        BinOpEq(And)
                    },
                    _ => BinOp(And)
                }
            },

            '|' => {
                match self.peek() {
                    '|' => {
                        self.advance();
                        OrOr
                    },
                    '=' => {
                        self.advance();
                        BinOpEq(Or)
                    },
                    _ => BinOp(Or)
                }
            },

            '+' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        BinOpEq(Plus)
                    },
                    _ => BinOp(Plus)
                }
            },
            '-' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        BinOpEq(Minus)
                    },
                    _ => BinOp(Plus)
                }
            },
            '*' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        BinOpEq(Star)
                    },
                    _ => BinOp(Star)
                }
            },
            '/' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        BinOpEq(Slash)
                    },
                    _ => BinOp(Slash)
                }
            },

            '>' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        GreaterEq
                    },
                    _ => Greater
                }
            },
            '=' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        EqEq
                    },
                    _ => Eq
                }
            },
            '<' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        LessEq
                    },
                    _ => Less
                }
            },

            '!' => {
                match self.peek() {
                    '=' => {
                        self.advance();
                        NotEq
                    },
                    _ => Bang
                }
            },

            '"' => {
                let terminated = self.double_quoted_string();
                let suffix_start = self.idx - start_idx;
                Literal { kind: LiteralKind::Str(terminated), suffix_start }
            },

            ';' => Semicolon,
            ',' => Comma,
            '~' => Tilde,
            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '{' => OpenCurly,
            '}' => CloseCurly,

            _ => Unknown
        };


        let end_idx = self.idx;

        Token::new(&self.src[start_idx..end_idx], kind, line, column)
    }
}

#[inline(always)]
fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

#[inline(always)]
fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || c.is_numeric()
}


#[allow(dead_code)]
pub struct Tokenizer<'src> {
    stream: TokenStream<'src>
}

impl<'src> Tokenizer<'src> {
    fn new(input: &'src str) -> Self {
        let stream = TokenStream::new(input);

        Tokenizer {
            stream
        }
    }
}

impl<'src> Iterator for Tokenizer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Token<'src>> {
        let token = self.stream.next_token();
        if token.kind != Eof { Some(token) } else { None }
    }
}

pub fn tokenize(input: &str) -> Tokenizer<'_> {
    Tokenizer::new(input)
}

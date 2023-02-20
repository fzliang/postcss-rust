use std::rc::{Rc, Weak};

use onig::Regex;

use super::input::Input;

const TAB: u8 = b'\x09'; // "'"
const NEWLINE: u8 = b'\x0A'; // '\n'
const FEED: u8 = b'\x0C'; // '\f'
const CR: u8 = b'\x0D'; // '\r'
const SPACE: u8 = b'\x20'; // ' '
const DOUBLE_QUOTE: u8 = b'\x22'; // '"'
const SINGLE_QUOTE: u8 = b'\x27'; // "'"
const OPEN_PARENTHESES: u8 = b'\x28'; // '('
const CLOSE_PARENTHESES: u8 = b'\x29'; // ')'
const ASTERISK: u8 = b'\x2A'; // '*'
const SLASH: u8 = b'\x2F'; // '/'
const COLON: u8 = b'\x3A'; // ':'
const SEMICOLON: u8 = b'\x3B'; //  ';'
const AT: u8 = b'\x40'; // @
const OPEN_SQUARE: u8 = b'\x5B'; // '['
const BACKSLASH: u8 = b'\x5C'; // '\\'
const CLOSE_SQUARE: u8 = b'\x5D'; // ']'
const OPEN_CURLY: u8 = b'\x7B'; // '{'
const CLOSE_CURLY: u8 = b'\x7D'; // '}'

const RE_AT_END: &str = r#"[\t\n\f\r "\#'()/;[\\]{}]"#;
const RE_WORD_END: &str = r#"[\t\n\f\r !"\#'():;@[\\]{}]|/(?=\*)"#;
const RE_BAD_BRACKET: &str = r#".[\n"'(/\\]"#;
const RE_HEX_ESCAPE: &str = r#"[\da-fA-F]/"#;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    token_type: &'a str,
    css: &'a str,
    pos: usize,
    next: usize,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    css: &'a str,
    ignore: bool,
    pos: usize,
    next: usize,
    length: usize,
    returned: Vec<Rc<Box<Token<'a>>>>,
    buffer: Vec<Rc<Box<Token<'a>>>>,
    re_word_end: Regex,
    re_bad_bracket: Regex,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a Input) -> Tokenizer<'a> {
        Self {
            css: input.css,
            ignore: false,
            pos: 0,
            next: 0,
            length: input.css.len(),
            returned: Vec::new(),
            buffer: Vec::new(),
            re_word_end: Regex::new(RE_WORD_END).unwrap(),
            re_bad_bracket: Regex::new(RE_BAD_BRACKET).unwrap(),
        }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn unclosed(msg: &str) {
        // todo!("抛出异常");
        println!("抛出异常");
    }

    pub fn end_of_file(&self) -> bool {
        self.returned.len() == 0 && self.pos >= self.length
    }

    pub fn next_token(&mut self) -> Option<Rc<Box<Token<'a>>>> {
        if self.returned.len() > 0 {
            return self.returned.pop();
        }

        if self.pos >= self.length {
            return None;
        }

        let ignoreUnclosed = false;

        let code = self.css.as_bytes()[self.pos];

        let mut currentToken: Option<Rc<Box<Token>>> = None;

        match code {
            NEWLINE | SPACE | TAB | CR | FEED => {
                self.next = self.pos;

                loop {
                    self.next += 1;

                    if self.next >= self.css.len() {
                        break;
                    }

                    let _code = self.css.as_bytes()[self.next];

                    if (_code != SPACE
                        && _code != NEWLINE
                        && _code != TAB
                        && _code != CR
                        && _code != FEED)
                        || self.next >= self.css.len()
                    {
                        break;
                    }
                }

                currentToken = Some(Rc::new(Box::new(Token {
                    token_type: "space",
                    css: &self.css[self.pos..self.next],
                    pos: self.pos,
                    next: self.next - 1,
                })));

                self.pos = self.next - 1
            }

            OPEN_SQUARE | CLOSE_SQUARE | OPEN_CURLY | CLOSE_CURLY | COLON | SEMICOLON
            | CLOSE_PARENTHESES => {
                let control_char = match code {
                    OPEN_SQUARE => "[",
                    CLOSE_SQUARE => "]",
                    OPEN_CURLY => "{",
                    CLOSE_CURLY => "}",
                    COLON => ":",
                    SEMICOLON => ";",
                    CLOSE_PARENTHESES => ")",
                    _ => "_",
                };

                currentToken = Some(Rc::new(Box::new(Token {
                    token_type: control_char,
                    css: control_char,
                    pos: self.pos,
                    next: self.pos + 1,
                })));
            }

            OPEN_PARENTHESES => {
                let prev = if let Some(item) = self.buffer.pop() {
                    item.css
                } else {
                    ""
                };

                let n = self.css.as_bytes()[self.pos + 1];

                if prev == "url"
                    && n != SINGLE_QUOTE
                    && n != DOUBLE_QUOTE
                    && n != SPACE
                    && n != NEWLINE
                    && n != TAB
                    && n != FEED
                    && n != CR
                {
                    self.next = self.pos;

                    let mut escaped: bool;
                    loop {
                        escaped = false;

                        if let Some(index) = self.css[self.next + 1..].find(")") {
                            self.next = self.next + 1 + index;
                        } else {
                            if self.ignore || ignoreUnclosed {
                                self.next = self.pos + 1;
                                break;
                            } else {
                                Tokenizer::<'a>::unclosed("string");
                            }
                        }

                        let mut escape_pos = self.next;
                        while self.css.as_bytes()[escape_pos - 1] == BACKSLASH {
                            escape_pos -= 1;
                            escaped = !escaped;
                        }

                        if !escaped {
                            break;
                        }
                    }

                    currentToken = Some(Rc::new(Box::new(Token {
                        token_type: "brackets",
                        css: &self.css[self.pos..self.next + 1],
                        pos: self.pos,
                        next: self.next,
                    })));

                    self.pos = self.next;
                } else {
                    let mut find_close_parentheses = false;
                    if let Some(index) = self.css[self.pos + 1..].find(")") {
                        self.next = self.pos + 1 + index;
                        find_close_parentheses = true;
                    } else {
                        find_close_parentheses = false;
                    };

                    let content = if find_close_parentheses {
                        &self.css[self.pos..self.next + 1]
                    } else {
                        ""
                    };

                    // let re = Regex::new(RE_BAD_BRACKET).unwrap();

                    if !find_close_parentheses || self.re_bad_bracket.find(content).is_some() {
                        currentToken = Some(Rc::new(Box::new(Token {
                            token_type: "(",
                            css: "(",
                            pos: self.pos,
                            next: self.pos,
                        })));
                    } else {
                        currentToken = Some(Rc::new(Box::new(Token {
                            token_type: "brackets",
                            css: content,
                            pos: self.pos,
                            next: self.next,
                        })));
                        self.pos = self.next;
                    }
                }
            }

            SINGLE_QUOTE | DOUBLE_QUOTE => {
                let quote = if code == SINGLE_QUOTE { "'" } else { r#"""# };
                self.next = self.pos;

                let mut escaped: bool;
                loop {
                    escaped = false;

                    if let Some(index) = self.css[self.next + 1..].find(quote) {
                        self.next = self.next + 1 + index;
                    } else {
                        if self.ignore || ignoreUnclosed {
                            self.next = self.pos + 1;
                            break;
                        } else {
                            Tokenizer::<'a>::unclosed("string");
                        }
                    }

                    let mut escape_pos = self.next;
                    while self.css.as_bytes()[escape_pos - 1] == BACKSLASH {
                        escape_pos -= 1;
                        escaped = !escaped;
                    }

                    if !escaped {
                        break;
                    }
                }

                currentToken = Some(Rc::new(Box::new(Token {
                    token_type: "string",
                    css: &self.css[self.pos..self.next + 1],
                    pos: self.pos,
                    next: self.next,
                })));

                self.pos = self.next;
            }

            AT => {}

            BACKSLASH => {}

            _ => {
                // let re = Regex::new(RE_WORD_END).unwrap();

                if let Some((_, mat_end)) = self.re_word_end.find(&self.css[self.pos + 1..]) {
                    self.next = self.pos + mat_end - 1;
                } else {
                    self.next = self.pos + 1;
                }

                let css = &self.css[self.pos..self.next + 1];

                let _token = Rc::new(Box::new(Token {
                    token_type: "word",
                    css,
                    pos: self.pos,
                    next: self.next,
                }));

                self.buffer.push(_token.clone());

                currentToken = Some(_token.clone());

                self.pos = self.next;
            }
        }

        self.pos += 1;

        currentToken
    }

    pub fn back(&mut self, token: &Rc<Box<Token<'a>>>) {
        self.returned.push(token.clone());
    }
}

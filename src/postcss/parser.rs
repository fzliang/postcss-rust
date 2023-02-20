use std::{
    borrow::Borrow,
    rc::{Rc, Weak},
};

use crate::postcss::tokenize::Token;

use super::{
    input::Input,
    node::{Position, Source},
    node_type::NodeType,
    root::Root,
    tokenize::Tokenizer,
};

#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a Input<'a>,

    root: Rc<Box<Root<'a>>>,

    // current: Weak<Box<NodeType<'a>>>,
    spaces: &'a str,

    semicolon: bool,

    customProperty: bool,

    source: Source<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Input<'a>) -> Self {
        let root = Rc::new(Box::new(Root::new()));

        let mut t = Tokenizer::new(input);
        let mut tokenVec: Vec<Weak<Box<Token>>> = Vec::new();

        while !t.end_of_file() {
            if let Some(token) = t.next_token() {
                tokenVec.push(Rc::downgrade(&token));
            }
        }

        // dbg!(tokenVec);

        Self {
            input,
            root,
            // current: NodeType::Root(Box::new(Root::new())),
            spaces: "",
            semicolon: false,
            customProperty: false,
            source: Source::new(input, Position::new(0, 1, 1), None),
        }
    }
}

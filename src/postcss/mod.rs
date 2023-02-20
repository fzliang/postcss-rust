mod container;
mod input;
mod node;
mod parser;
mod root;
mod node_type;
mod tokenize;

use self::input::*;
use self::parser::*;

pub fn parse(css: &str) {
    let input = Input::new(css);
    let parser = Parser::new(&input);
    // println!("css: {:#?}", parser);
}

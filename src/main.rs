extern crate combine;

use combine::{choice, error::ParseError, parser::char::char, Parser, Stream};

mod ast;
use crate::ast::ast::{Ast, Heading};



fn main() {
    println!("aaa")
    // Ok((Prod(Scalar(3.0), Sum(Scalar(1.0), Scalar(2.0))), ""))
}

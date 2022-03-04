#[macro_use]
extern crate combine;
use combine::{
    between, chainl1, choice,
    error::ParseError,
    many1,
    parser::char::{char, digit, letter, spaces},
    Parser, Stream,
};

#[derive(Debug)]
pub enum Expr {
    Scalar(f64),
    Var(char),
    Prod(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sum(Box<Expr>, Box<Expr>),
    Diff(Box<Expr>, Box<Expr>),
}

fn parse_expr<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = choice([char('-'), char('+')]).map(|op| {
        move |a, b| {
            if op == '+' {
                Expr::Sum(Box::new(a), Box::new(b))
            } else if op == '-' {
                Expr::Diff(Box::new(a), Box::new(b))
            } else {
                unimplemented!()
            }
        }
    });
    let mut sum = chainl1(parse_term(), tok);
    sum
}

fn parse_term<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = choice([char('*'), char('/')]).map(|op| {
        move |a, b| {
            if op == '*' {
                Expr::Prod(Box::new(a), Box::new(b))
            } else if op == '/' {
                Expr::Div(Box::new(a), Box::new(b))
            } else {
                unimplemented!()
            }
        }
    });
    let mut prod = chainl1(parse_factor_(), tok);
    prod
}

parser! {
    pub fn parse_factor_[Input]()(Input) -> Expr
    where [Input: Stream<Token=char>, Input::Error: ParseError<Input::Token, Input::Range, Input::Position>]
    {
        let scalar = many1(digit()).map(|t: String| Expr::Scalar(t.parse().unwrap()));
        let var = letter().map(Expr::Var);
        let parens = between(char('('), char(')'), parse_expr());
        let p = spaces().with(scalar.or(var).or(parens));
        p
    }
}

fn main() {
    let res = parse_expr().parse("3*(1+2)");
    println!("{:?}", res)
    // Ok((Prod(Scalar(3.0), Sum(Scalar(1.0), Scalar(2.0))), ""))
}

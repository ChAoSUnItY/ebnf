use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while1},
    character::complete,
    combinator::complete,
    error::VerboseError,
    multi::many1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{node::SymbolKind, Expression, Grammar, Node};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn parse_lhs(input: &str) -> Res<&str, &str> {
    let (input, lhs) = preceded(complete::multispace0, complete::alphanumeric1)(input)?;
    let (input, _) = preceded(complete::multispace0, alt((tag("="), tag("::="))))(input)?;

    Ok((input, lhs.trim_end()))
}

fn parse_rhs(input: &str) -> Res<&str, Node> {
    let (input, rhs) = preceded(
        complete::multispace0,
        terminated(parse_multiple, complete::char(';')),
    )(input)?;

    Ok((input, rhs))
}

fn parse_string(input: &str) -> Res<&str, Node> {
    let (input, string) = alt((
        delimited(complete::char('\''), take_until("'"), complete::char('\'')),
        delimited(complete::char('"'), take_until("\""), complete::char('"')),
    ))(input)?;

    Ok((input, Node::String(string.to_string())))
}

fn parse_int(input: &str) -> Res<&str, Node> {
    let (input, int) = preceded(
        complete::multispace0,
        terminated(complete::digit1, complete::multispace0),
    )(input)?;

    Ok((
        input,
        Node::Int(
            int.parse::<i32>()
                .expect("Unable to parse int, this should not happen"),
        ),
    ))
}

fn parse_terminal(input: &str) -> Res<&str, Node> {
    let (input, symbol) = preceded(
        complete::multispace0,
        terminated(complete::alphanumeric1, complete::multispace0),
    )(input)?;

    Ok((input, Node::Terminal(symbol.to_string())))
}

fn parse_multiple(input: &str) -> Res<&str, Node> {
    let (input, node) = preceded(complete::multispace0, many1(parse_node))(input)?;

    match node {
        _ if node.len() == 1 => Ok((input, node[0].clone())),
        _ => Ok((input, Node::Multiple(node))),
    }
}

fn parse_node(input: &str) -> Res<&str, Node> {
    let (input, left_node) = preceded(
        complete::multispace0,
        alt((parse_group, parse_string, parse_int, parse_terminal)),
    )(input)?;

    let optional_symbol: Res<&str, (SymbolKind, Node)> =
        preceded(complete::multispace0, parse_symbol)(input);

    match optional_symbol {
        Ok((input, (symbol, right_node))) => Ok((
            input,
            Node::Symbol(Box::new(left_node), symbol, Box::new(right_node)),
        )),
        Err(_) => Ok((input, left_node)),
    }
}

fn parse_symbol(input: &str) -> Res<&str, (SymbolKind, Node)> {
    let (input, symbol_pair) = preceded(
        complete::multispace0,
        alt((
            parse_concatenation,
            parse_alternation,
            parse_exception,
            parse_repetition,
        )),
    )(input)?;

    Ok((input, symbol_pair))
}

fn parse_concatenation(input: &str) -> Res<&str, (SymbolKind, Node)> {
    let (input, node) = preceded(complete::char(','), parse_multiple)(input)?;

    Ok((input, (SymbolKind::Concatenation, node)))
}

fn parse_alternation(input: &str) -> Res<&str, (SymbolKind, Node)> {
    let (input, node) = preceded(complete::char('|'), parse_multiple)(input)?;

    Ok((input, (SymbolKind::Alternation, node)))
}

fn parse_exception(input: &str) -> Res<&str, (SymbolKind, Node)> {
    let (input, node) = preceded(complete::char('-'), parse_multiple)(input)?;

    Ok((input, (SymbolKind::Exception, node)))
}

fn parse_repetition(input: &str) -> Res<&str, (SymbolKind, Node)> {
    let (input, node) = preceded(complete::char('*'), parse_multiple)(input)?;

    Ok((input, (SymbolKind::Repetition, node)))
}

fn parse_group(input: &str) -> Res<&str, Node> {
    let (input, inner) = preceded(
        complete::multispace0,
        delimited(complete::char('('), take_until(")"), complete::char(')')),
    )(input)?;
    let (_, node) = preceded(complete::multispace0, parse_multiple)(inner)?;

    Ok((input, Node::Group(Box::new(node))))
}

fn parse_expressions(input: &str) -> Res<&str, Vec<Expression>> {
    let mut source = input;
    let mut expressions = Vec::<Expression>::new();

    while !source.is_empty() {
        let (input, lhs) = parse_lhs(source)?;
        let (input, rhs) = parse_rhs(input)?;

        expressions.push(Expression {
            lhs: lhs.to_string(),
            rhs,
        });

        source = input.trim_start();
    }

    Ok((input, expressions))
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let src = r"
            a ::= ('b' 'c' * 3) 'b';
            b ::= c;
        ";

        let (input, vec) = parse_expressions(src).unwrap();

        println!("{:?}", vec);
    }
}

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete,
    error::{VerboseError, VerboseErrorKind},
    multi::many1,
    sequence::{delimited, preceded, terminated},
    Err, IResult,
};

use parse_hyperlinks::take_until_unbalanced;

use crate::{
    node::{RegexExtKind, SymbolKind},
    Expression, Grammar, Node,
};

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

fn parse_regex_string(input: &str) -> Res<&str, Node> {
    let (input, string) = alt((
        delimited(tag("#'"), take_until("'"), complete::char('\'')),
        delimited(tag("#\""), take_until("\""), complete::char('"')),
    ))(input)?;

    Ok((input, Node::RegexString(string.to_string())))
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
    let (mut input, mut left_node) = preceded(
        complete::multispace0,
        alt((
            parse_group,
            parse_optional,
            parse_repeat,
            parse_string,
            parse_regex_string,
            parse_terminal,
        )),
    )(input)?;

    let optional_regex_ext: Res<&str, RegexExtKind> = parse_regex_ext(input);

    match optional_regex_ext {
        Ok(((s, regex_ext_kind))) => {
            input = s;
            left_node = Node::RegexExt(Box::new(left_node), regex_ext_kind);
        }
        Err(_) => {}
    }

    let optional_symbol: Res<&str, (SymbolKind, Node)> = parse_symbol(input);

    match optional_symbol {
        Ok((input, (symbol, right_node))) => Ok((
            input,
            Node::Symbol(Box::new(left_node), symbol, Box::new(right_node)),
        )),
        Err(_) => Ok((input, left_node)),
    }
}

fn parse_regex_ext(input: &str) -> Res<&str, RegexExtKind> {
    let (input, regex_ext) = preceded(
        complete::multispace0,
        alt((
            complete::char('*'),
            complete::char('+'),
            complete::char('?'),
        )),
    )(input)?;

    let regex_kind = match regex_ext {
        '*' => RegexExtKind::Repeat0,
        '+' => RegexExtKind::Repeat1,
        '?' => RegexExtKind::Optional,
        _ => unreachable!("Unexpected regex extension symbol. this should not happen"),
    };

    Ok((input, regex_kind))
}

fn parse_symbol(input: &str) -> Res<&str, (SymbolKind, Node)> {
    let (input, symbol_pair) = preceded(
        complete::multispace0,
        alt((parse_concatenation, parse_alternation)),
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

fn parse_delimited_node(
    input: &str,
    opening_bracket: char,
    closing_bracket: char,
) -> Res<&str, &str> {
    let result = delimited(
        tag(opening_bracket.to_string().as_str()),
        take_until_unbalanced(opening_bracket, closing_bracket),
        tag(closing_bracket.to_string().as_str()),
    )(input);

    match result {
        Ok((input, inner)) => Ok((input, inner)),
        Err(_) => Err(Err::Error(VerboseError {
            errors: vec![(
                input,
                VerboseErrorKind::Context("Incomplete delimited node"),
            )],
        })),
    }
}

fn parse_group(input: &str) -> Res<&str, Node> {
    let (input, inner) = parse_delimited_node(input, '(', ')')?;
    let (_, node) = preceded(complete::multispace0, parse_multiple)(inner)?;

    Ok((input, Node::Group(Box::new(node))))
}

fn parse_optional(input: &str) -> Res<&str, Node> {
    let (input, inner) = parse_delimited_node(input, '[', ']')?;
    let (_, node) = preceded(complete::multispace0, parse_multiple)(inner)?;

    Ok((input, Node::Optional(Box::new(node))))
}

fn parse_repeat(input: &str) -> Res<&str, Node> {
    let (input, inner) = parse_delimited_node(input, '{', '}')?;
    let (_, node) = preceded(complete::multispace0, parse_multiple)(inner)?;

    Ok((input, Node::Repeat(Box::new(node))))
}

pub(crate) fn parse_expressions(input: &str) -> Res<&str, Vec<Expression>> {
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
            Filter ::= ( First ' ' )? ( Number '~ ' )? ( Number '-' Number ) ( ' ' Number '~' )? ( ' Hz' )? ( ' B' )? ( ' I' )? ( ' A' )?;
            First  ::= #'[A-Za-z][A-Za-z0-9_+]*';
            Number ::= Digits ( ( '.' | ',' ) Digits? )?;
            Digits ::= #'[0-9]+';
        ";

        let (input, vec) = parse_expressions(src).unwrap();

        println!("{:#?}", vec);
    }
}

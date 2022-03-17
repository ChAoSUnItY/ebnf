use nom::{error::VerboseError, IResult};

use crate::{Expression, Grammar, Node};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

// fn expression_complete(input: &str) -> Res<&str, Expression> {
//     context(
//         "expression",
//         separated_pair(first, sep, second)
// )(input)
// }

// fn grammar(input: &str) -> Res<&str, Grammar>  {
//     let (input, _) = peek(expression)(input)?;
//     let (input, exprs) = many1(complete(expression))(input)?;

//     Ok((input, Grammar::new(exprs)))
// }

// fn grammar_complete(input: &str) -> Res<&str, Grammar> {
//     let (input, g) = all_consuming(grammar)(input)?;

//     Ok((input, g))
// }

#[cfg(test)]
mod test {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while1},
        character::{complete, is_alphanumeric},
        error::{Error, VerboseErrorKind},
        sequence::{delimited, preceded, terminated}, combinator::not,
    };

    use super::*;

    fn parse_lhs(input: &str) -> Res<&str, &str> {
        let (input, lhs) = preceded(
            complete::multispace0,
            terminated(complete::alphanumeric1, alt((tag("="), complete::multispace1))),
        )(input)?;

        Ok((input, lhs.trim()))
    }

    fn parse_rhs(input: &str) -> Res<&str, Res<&str, Node>> {
        let (input, rhs) = preceded(
            take_while1(|c: char| c.is_whitespace() || c == '='),
            terminated(take_until(";"), tag(";")),
        )(input)?;

        Ok((input, parse_node(rhs)))
    }

    fn parse_string(input: &str) -> Res<&str, Node> {
        let (input, str) = alt((
            delimited(complete::char('\''), take_until("'"), complete::char('\'')),
            delimited(complete::char('"'), take_until("\""), complete::char('"')),
        ))(input)?;

        Ok((input, Node::String(str.to_string())))
    }

    fn parse_symbol(input: &str) -> Res<&str, Node> {
        let (input, symbol) = preceded(complete::multispace0, terminated(complete::alphanumeric1, complete::multispace0))(input)?;
    
        Ok((input, Node::Symbol(symbol.to_string())))
    }

    fn parse_node(input: &str) -> Res<&str, Node> {
        let (input, node) = preceded(complete::multispace0, alt((parse_string, parse_symbol)))(input)?;

        Ok((input, node))
    }

    fn parse_expressions(input: &str) -> Res<&str, Vec<Expression>> {
        let mut source = input;
        let mut expressions = Vec::<Expression>::new();

        while !source.is_empty() {
            let (input, lhs) = parse_lhs(source)?;
            let (input, rhs) = parse_rhs(input)?;

            match rhs {
                Ok(node_result) => {
                    let (_, node) = node_result;

                    expressions.push(Expression {
                        lhs: lhs.to_string(),
                        rhs: vec![node],
                    });
                }
                Err(err) => {
                    return Err(err);
                }
            }

            source = input.trim_start();
        }

        Ok((input, expressions))
    }

    #[test]
    fn test_parse() {
        let src = r"
            a = b;
            b = c;
        ";

        let (input, vec) = parse_expressions(src).unwrap();

        println!("{:?}", vec);
    }
}

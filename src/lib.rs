//! ebnf - A successor bnf parsing library of bnf parsing library, for parsing Extended Backus–Naur form context-free grammars
//! 
//! The code is available on [GitHub](https://github.com/ChAoSUnItY/ebnf)
//! 
//! ## Disclaimer:
//! There are various variants of EBNF, which uses somewhat different syntactic conventions. This library 
//! takes [EBNF Evaluator](https://mdkrajnak.github.io/ebnftest/)'s example code as standard, which has 
//! almost most syntactic conventions on Wikipedia's page.
//! 
//! ## What does a valid EBNF grammar looks like?
//! 
//! The following example is taken from EBNF Evaluator:
//! 
//! ```ebnf
//! filter ::= ( first ' ' )? ( number '~ ' )? ( number '-' number ) ( ' ' number '~' )? ( ' hz' )? ( ' b' )? ( ' i' )? ( ' a' )?;
//! first  ::= #'[a-za-z][a-za-z0-9_+]*';
//! number ::= digits ( ( '.' | ',' ) digits? )?;
//! digits ::= #'[0-9]+';
//! ```
//! 
//! ## How to use this library?
//! 
//! ```rust
//! extern crate ebnf;
//! 
//! fn main() {
//!     let source = r"
//!         filter ::= ( first ' ' )? ( number '~ ' )? ( number '-' number ) ( ' ' number '~' )? ( ' hz' )? ( ' b' )? ( ' i' )? ( ' a' )?;
//!         first  ::= #'[a-za-z][a-za-z0-9_+]*';
//!         number ::= digits ( ( '.' | ',' ) digits? )?;
//!         digits ::= #'[0-9]+';
//!     ";
//! 
//!     let result = ebnf::get_grammar(source);
//! }
//! ```

extern crate nom;
extern crate parse_hyperlinks;
mod expression;
mod grammar;
mod node;
mod parser;
pub use expression::Expression;
pub use grammar::Grammar;
pub use node::{Node, SymbolKind, RegexExtKind};

// get and parse EBNF grammar source into data structure ebnf::Grammar
pub fn get_grammar(input: &str) -> Result<Grammar, nom::Err<nom::error::VerboseError<&str>>> {
    match parser::parse_expressions(input) {
        Ok((_, expressions)) => Ok(Grammar { expressions }),
        Err(e) => Err(e),
    }
}

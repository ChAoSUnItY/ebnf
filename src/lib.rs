#![cfg_attr(feature = "strict", deny(warnings))]

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
pub use node::{Node, RegexExtKind, SymbolKind};

/// Get and parse EBNF grammar source into [Grammar], returns [Err] when given grammar is invalid.
/// 
/// # Example
/// 
/// ```rust
/// use ebnf::get_grammar;
/// 
/// let grammar_literal = r"
///     term ::= '1';
/// ";
/// let grammar = get_grammar(grammar_literal)?;
/// ```
pub fn get_grammar(input: &str) -> Result<Grammar, nom::Err<nom::error::VerboseError<&str>>> {
    parser::parse_expressions(input).map(|(_, expressions)| Grammar { expressions })
}

/*!
 * This crate provides an alternative implementation of a toy Regex engine that uses Brzozowski derivatives
 * instead of building an automaton (either NFA or a DFA).
 * This implementation closely follows the very educational [Python implementation] and provides similar
 * APIs to it. Note that the set of supported regular expressions in general is minimal but in line with the Python implementation.
 *
 * The recommended way to build an [`Expr`] out of a regular expression is via [`std::string::FromStr`]. For example:
 *
 * ```rust
 * use brzozowski::Expr;
 *
 * let re = "b(a*)(na)*".parse::<Expr>().unwrap();
 * assert!(re.is_match("banana"));
 * ```
 *
 * [Python implementation]: https://github.com/aalekhpatel07/brzozowski/tree/main?tab=readme-ov-file#brzozowski-derivative
 *
 * The [`Expr`]'s are essentially a tree representation of the regular expression where the following operations are possible:
 * - [`Expr::Concat`]\: Concatenate two strings together (i.e. `<expr>·<expr>`)
 * - [`Expr::Union`]\: Take union of two strings (i.e. `<expr>|<expr>`)
 * - [`Expr::Kleene`]\: Describe the Kleene star for a string (i.e. `<expr>*`).
 * - [`Expr::Term`]\: The leaf nodes in the tree that represent characters of the alphabet.
 *
 * To explore how the derivatives are calculated, check out the [`Expr::derivative`], [`Expr::nulled`], and [`Expr::simplify`] methods.
 *
 */

mod expr;
pub use expr::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PREC: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('(', 0);
        m.insert('|', 1);
        m.insert('·', 2);
        m.insert('*', 3);
        m
    };
}

/// An iterator over the characters of the
/// augmented regular expression, i.e. where the concatenation
/// is injected wherever it is provided in regular expression
/// implicitly.
#[derive(Debug)]
pub struct Augment<I> {
    src: I,
    prev: Option<char>,
    curr: Option<char>,
    index: Option<usize>,
    check_prev_curr: bool,
    complete: bool,
    yield_curr: bool,
}

impl<I> Augment<I>
where
    I: Iterator<Item = char>,
{
    /// Construct a new Augment from an iterator over
    /// characters of a regular expression.
    pub fn new(src: I) -> Self {
        Self {
            src,
            prev: None,
            curr: None,
            index: None,
            check_prev_curr: false,
            complete: false,
            yield_curr: false,
        }
    }
}

impl<I> Iterator for Augment<I>
where
    I: Iterator<Item = char>,
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            return None;
        }
        if self.yield_curr {
            self.yield_curr = false;
            return self.curr;
        }
        match self.src.next() {
            None => {
                // EOS when empty string.
                if self.index.is_none() {
                    // mark completion.
                    self.complete = true;
                    return Some('ε');
                }
                None
            }
            Some(char) => {
                let mut index = self.index.unwrap_or_default();
                if index > 0 {
                    self.prev = self.curr;
                }
                self.curr = Some(char);
                index += 1;
                self.index = Some(index);
                if index > 1 {
                    // Now start checking windows.
                    self.check_prev_curr = true;
                }
                if self.check_prev_curr
                    && !(matches!(self.prev, Some('(' | '|'))
                        || matches!(self.curr, Some('|' | ')' | '*')))
                {
                    self.yield_curr = true;
                    Some('·')
                } else {
                    self.curr
                }
            }
        }
    }
}

/// Extend the input regex with an explicit concatenation operator (`·`).
///
/// # Examples
///
/// ```
/// use brzozowski;
///
/// let s = "ba*(n|a)*";
/// let augmented = brzozowski::augment(s.chars()).collect::<Vec<char>>();
/// assert_eq!(String::from_iter(augmented), "b·a*·(n|a)*");
/// ```
pub fn augment<I>(src: I) -> impl Iterator<Item = char>
where
    I: Iterator<Item = char>,
{
    Augment::new(src)
}

/// Extend the input regex with an explicit concatenation operator (`·`).
/// This is an imperative implementation as compared to the iterative implementation
/// of [`augment`].
///
/// # Examples
///
/// ```
/// use brzozowski;
///
/// let s = "ba*(n|a)*";
/// let chars = s.chars().collect::<Vec<char>>();
/// let augmented = brzozowski::augment_imperative(&chars);
/// assert_eq!(String::from_iter(augmented), "b·a*·(n|a)*");
/// ```
pub fn augment_imperative(src: &[char]) -> Vec<char> {
    if src.is_empty() {
        return vec!['ε'];
    }
    if src.len() == 1 {
        return src.to_vec();
    }
    let mut dest = vec![];
    for i in 0..src.len() {
        if i > 0 && !("|)*".contains(src[i]) || "(|".contains(src[i - 1])) {
            dest.push('·');
        }
        dest.push(src[i]);
    }
    dest
}

/// Use the Shunting Yard algorithm to convert an infix expression to a postfix expression.
/// This is the intermediate step to convert an infix regular expression into an [`Expr`];
///
/// # Examples
///
/// ```
/// use brzozowski;
///
/// let s = "ba*(n|a)*";
///
/// let augmented = brzozowski::augment(s.chars()).collect::<Vec<char>>();
/// assert_eq!(String::from_iter(&augmented), "b·a*·(n|a)*");
///
/// let postfix = brzozowski::infix_to_postfix(&augmented).unwrap();
/// assert_eq!(String::from_iter(postfix), "ba*·na|*·");
/// ```
pub fn infix_to_postfix(expression: &[char]) -> Result<Vec<char>, String> {
    let mut stack = vec![];
    let mut output = vec![];

    for &c in expression {
        if c.is_alphanumeric() {
            output.push(c);
        } else if c == '(' {
            stack.push(c);
        } else if c == ')' {
            while !stack.is_empty() && !matches!(stack.last(), Some('(')) {
                output.push(stack.pop().unwrap());
            }
            stack.pop().unwrap();
        } else {
            while !stack.is_empty() {
                let last_prec = *PREC.get(stack.last().unwrap()).unwrap();
                let curr_prec = *PREC.get(&c).unwrap();
                if last_prec >= curr_prec {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(c);
        }
    }

    while let Some(item) = stack.pop() {
        output.push(item);
    }

    Ok(output)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
/// A tree representation of a regular expression.
pub enum Expr {
    /// Leaf nodes that are terms of an alphabet.
    Term(char),
    /// A concatenation of two expressions (i.e. `<expr>` · `<expr>`)
    Concat(Box<Expr>, Box<Expr>),
    /// The Kleene star for an expression (i.e. `<expr>*`)
    Kleene(Box<Expr>),
    /// The Union between two expressions (i.e. `<expr> | <expr>`)
    Union(Box<Expr>, Box<Expr>),
    /// The unique string of length zero.
    Epsilon,
    /// A marker for an empty set of strings spanned by this expression.
    Empty,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Term(term) => write!(f, "{}", term),
            Self::Concat(left, right) => write!(f, "{}·{}", left, right),
            Self::Kleene(base) => write!(f, "{}*", base),
            Self::Union(left, right) => write!(f, "{}|{}", left, right),
            Self::Epsilon => write!(f, "ε"),
            Self::Empty => write!(f, "∅"),
        }
    }
}

impl std::str::FromStr for Expr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let augmented = augment(s.chars()).collect::<Vec<_>>();
        let postfix = infix_to_postfix(&augmented)?;
        Self::parse_postfix(&postfix)
    }
}

impl Expr {
    /// Given a valid postfix expression, build an [`Expr`] out of it.
    /// # Examples
    /// ```
    /// use brzozowski::{self, Expr};
    ///
    /// let s = "ba*(n|a)*";
    ///
    /// let augmented = brzozowski::augment(s.chars()).collect::<Vec<char>>();
    /// assert_eq!(String::from_iter(&augmented), "b·a*·(n|a)*");
    ///
    /// let postfix = brzozowski::infix_to_postfix(&augmented).unwrap();
    /// assert_eq!(String::from_iter(&postfix), "ba*·na|*·");
    ///
    /// let expr = Expr::parse_postfix(&postfix).unwrap();
    /// let expected = Expr::Concat(
    ///     Box::new(Expr::Concat(
    ///         Box::new(Expr::Term('b')),
    ///         Box::new(Expr::Kleene(
    ///             Box::new(Expr::Term('a'))
    ///         ))
    ///     )),
    ///     Box::new(Expr::Kleene(
    ///         Box::new(Expr::Union(
    ///             Box::new(Expr::Term('n')),
    ///             Box::new(Expr::Term('a'))
    ///         ))
    ///     ))
    /// );
    /// assert_eq!(expr, expected);
    /// ```
    pub fn parse_postfix(s: &[char]) -> Result<Self, String> {
        let mut stack: Vec<Expr> = vec![];
        for &c in s {
            match c {
                '|' => {
                    let right = stack.pop().ok_or_else(|| "empty stack".to_string())?;
                    let left = stack.pop().ok_or_else(|| "empty stack".to_string())?;
                    stack.push(Expr::Union(Box::new(left), Box::new(right)));
                }
                '·' => {
                    let right = stack.pop().ok_or_else(|| "empty stack".to_string())?;
                    let left = stack.pop().ok_or_else(|| "empty stack".to_string())?;
                    stack.push(Expr::Concat(Box::new(left), Box::new(right)));
                }
                '*' => {
                    let base = stack.pop().ok_or_else(|| "empty stack".to_string())?;
                    stack.push(Expr::Kleene(Box::new(base)));
                }
                '∅' => {
                    stack.push(Expr::Empty);
                }
                'ε' => {
                    stack.push(Expr::Epsilon);
                }
                _ => {
                    stack.push(Expr::Term(c));
                }
            }
        }
        if stack.len() != 1 {
            return Err("expected stack to contain exactly one item at the end".to_string());
        }
        Ok(stack.pop().unwrap())
    }

    /// A utility function used to check if the language defined
    /// by this regular expression contains an empty string.
    /// This is almost equivalent to the [function `v(r)`] but with
    /// a slightly different signature.
    ///
    /// Instead of returning whether
    /// the regular expression contains the empty string directly,
    /// it returns a potentially simpler version of the regular expression
    /// that can be used to determine the answer to that question in a more
    /// efficient way.
    ///
    /// [function `v(r)`]: https://github.com/aalekhpatel07/brzozowski/tree/main?tab=readme-ov-file#definition-and-rules
    /// # Examples
    ///
    /// ```
    ///
    /// use brzozowski::Expr;
    ///
    /// let expr = "(c|b)".parse::<Expr>().unwrap();
    /// let nulled = expr.nulled();
    ///
    /// assert_eq!(
    ///     nulled,
    ///     Expr::Union(
    ///         Box::new(Expr::Empty),
    ///         Box::new(Expr::Empty)
    ///     )
    /// );
    /// ```
    pub fn nulled(&self) -> Expr {
        match self {
            Self::Empty => Expr::Empty,
            Self::Term(_) => Expr::Empty,
            Self::Epsilon => Expr::Epsilon,
            Self::Concat(left, right) => {
                Expr::Concat(Box::new((*left).nulled()), Box::new((*right).nulled()))
            }
            Self::Union(left, right) => {
                Expr::Union(Box::new((*left).nulled()), Box::new((*right).nulled()))
            }
            Self::Kleene(_) => Expr::Epsilon,
        }
    }

    /// Simplify the regular expression by collapsing
    /// operations involving an empty set or an Epsilon.
    ///
    /// # Example
    ///
    /// ```
    /// use brzozowski::Expr;
    ///
    /// let expr = "((c|b)ε)*".parse::<Expr>().unwrap();
    /// let simplified = expr.simplify();
    ///
    /// assert_eq!(
    ///     simplified,
    ///     Expr::Kleene(
    ///         Box::new(Expr::Union(
    ///             Box::new(Expr::Term('c')),
    ///             Box::new(Expr::Term('b'))
    ///         ))
    ///     )
    /// );
    /// ```
    ///
    pub fn simplify(&self) -> Expr {
        match self {
            Self::Concat(left, right) => match (left.as_ref(), right.as_ref()) {
                (Expr::Empty, _) => Expr::Empty,
                (_, Expr::Empty) => Expr::Empty,
                (Expr::Epsilon, x) => x.simplify(),
                (x, Expr::Epsilon) => x.simplify(),
                _ => Expr::Concat(Box::new(left.simplify()), Box::new(right.simplify())),
            },
            Self::Union(left, right) => match (left.as_ref(), right.as_ref()) {
                (Expr::Empty, x) => x.simplify(),
                (x, Expr::Empty) => x.simplify(),
                _ => Expr::Union(Box::new(left.simplify()), Box::new(right.simplify())),
            },
            Self::Kleene(base) => match base.as_ref() {
                Expr::Kleene(inner) => Expr::Kleene(Box::new(inner.simplify())),
                Expr::Empty => Expr::Epsilon,
                Expr::Epsilon => Expr::Epsilon,
                _ => Self::Kleene(Box::new(base.simplify())),
            },
            other => other.clone(),
        }
    }

    /// Repeatedly simplify an expression until we come across
    /// a representation that cannot be simplified any further.
    ///
    /// # Example
    ///
    /// ```
    /// use brzozowski::Expr;
    ///
    /// let expr = "((c|b)ε)*ε".parse::<Expr>().unwrap();
    /// let simplified = expr.simplify_to_end();
    ///
    /// assert_eq!(
    ///     simplified,
    ///     Expr::Kleene(
    ///         Box::new(Expr::Union(
    ///             Box::new(Expr::Term('c')),
    ///             Box::new(Expr::Term('b'))
    ///         ))
    ///     )
    /// );
    pub fn simplify_to_end(&self) -> Self {
        let mut prev = self.clone();
        let mut curr = self.simplify();
        while prev != curr {
            let simplified = curr.simplify();
            prev = curr;
            curr = simplified;
        }
        curr
    }

    /// Returns whether the language defined by this regular expression
    /// contains the empty string, Epsilon.
    ///
    /// # Examples
    ///
    /// ```
    /// use brzozowski::Expr;
    ///
    /// let expr = "((c|b)ε)*ε".parse::<Expr>().unwrap();
    /// assert!(expr.contains_epsilon());
    ///
    /// let expr = "a(b*)ε".parse::<Expr>().unwrap();
    /// assert!(!expr.contains_epsilon());
    /// ```
    pub fn contains_epsilon(&self) -> bool {
        self.nulled().simplify_to_end() == Expr::Epsilon
    }

    /// Following these [rules], compute the Brzozowski derivative of this
    /// regular expression with respect to the provided character.
    ///
    /// [rules]: https://github.com/aalekhpatel07/brzozowski/tree/main?tab=readme-ov-file#definition-and-rules
    ///
    /// # Examples
    /// ```
    /// use brzozowski::Expr;
    ///
    /// let s = "(c|b)at";
    /// let expr = s.parse::<Expr>().unwrap();
    ///
    /// let derivative = expr.derivative('c');
    /// assert_eq!(format!("{}", derivative), "ε|∅·a|∅|∅·∅·t|∅|∅·∅·∅");
    ///
    /// let simplified = derivative.simplify_to_end();
    /// assert_eq!(format!("{}", simplified), "a·t");
    ///
    /// let derivative = expr.derivative('e');
    /// assert_eq!(format!("{}", derivative), "∅|∅·a|∅|∅·∅·t|∅|∅·∅·∅");
    ///
    /// let simplified = derivative.simplify_to_end();
    /// assert_eq!(format!("{}", simplified), "∅");
    /// ```
    pub fn derivative(&self, c: char) -> Expr {
        match self {
            Self::Empty => Self::Empty,
            Self::Epsilon => Self::Empty,
            Self::Term(term) if *term == c => Self::Epsilon,
            Self::Term(_) => Self::Empty,
            Self::Concat(left, right) => Expr::Union(
                Box::new(Expr::Concat(Box::new(left.derivative(c)), right.clone())),
                Box::new(Expr::Concat(
                    Box::new(left.nulled()),
                    Box::new(right.derivative(c)),
                )),
            ),
            Self::Union(left, right) => {
                Expr::Union(Box::new(left.derivative(c)), Box::new(right.derivative(c)))
            }
            Self::Kleene(base) => Expr::Concat(
                Box::new(base.derivative(c)),
                Box::new(Expr::Kleene(base.clone())),
            ),
        }
    }

    /// Determine whether the provided pattern matches is accepted
    /// by this regular expression.
    ///
    /// # Examples
    /// ```
    /// use brzozowski::Expr;
    ///
    /// let regex = "(c|b)at";
    ///
    /// let expr = regex.parse::<Expr>().unwrap();
    /// assert!(expr.is_match("cat"));
    /// assert!(expr.is_match("bat"));
    /// assert!(!expr.is_match("pat"));
    /// ```
    pub fn is_match(&self, pat: &str) -> bool {
        let mut q = self.clone();
        for c in pat.chars() {
            q = q.derivative(c).simplify_to_end();
        }
        q.contains_epsilon()
    }
}

#[cfg(test)]
mod tests {
    use super::Expr;
    use crate::expr;
    use rand::distributions::{DistString, Standard};
    use test_case::test_case;

    fn random_string(len: usize) -> String {
        let alpha = Standard;
        let mut rng = rand::thread_rng();
        alpha.sample_string(&mut rng, len)
    }

    #[test_case("", "ε"; "empty string")]
    #[test_case("c", "c"; "single char")]
    #[test_case("(c|b)at", "(c|b)·a·t"; "simple")]
    #[test_case("(c|b)at(b*a)", "(c|b)·a·t·(b*·a)"; "complex")]
    fn augment(src: &str, expected: &str) {
        let observed = expr::augment(src.chars()).collect::<String>();
        assert_eq!(observed, expected);
    }

    #[test_case(""; "empty string")]
    #[test_case("c"; "single char")]
    #[test_case("(c|b)at"; "simple")]
    #[test_case("(c|b)at(b*a)"; "complex")]
    fn iterative(src: &str) {
        let observed_iterative = expr::augment(src.chars()).collect::<String>();
        let observed_imperative = expr::augment_imperative(&src.chars().collect::<Vec<_>>())
            .into_iter()
            .collect::<String>();
        assert_eq!(observed_imperative, observed_iterative);
    }

    fn compare_iterative_and_imperative(src: &str) {
        let observed_iterative = super::augment(src.chars()).collect::<String>();
        let observed_imperative = super::augment_imperative(&src.chars().collect::<Vec<_>>())
            .into_iter()
            .collect::<String>();
        assert_eq!(observed_imperative, observed_iterative);
    }

    #[test]
    fn random_regexes() {
        for len in 1..100 {
            for _ in 0..100 {
                compare_iterative_and_imperative(&random_string(len));
            }
        }
    }

    #[test_case("", "ε"; "empty string")]
    #[test_case("(c|b)at", "cb|a·t·"; "simple")]
    #[test_case("(c|b)(a|t)**|((ab))", "cb|at|**·ab·|"; "complex")]
    fn augmented_infix_to_postfix(src: &str, expected: &str) {
        let chars = src.chars().collect::<Vec<_>>();
        let augmented = expr::augment(chars.into_iter()).collect::<Vec<_>>();
        let tree = src.parse::<expr::Expr>();
        assert!(tree.is_ok());
        let observed = expr::infix_to_postfix(&augmented).unwrap();
        let expected = expected.chars().collect::<Vec<_>>();
        assert_eq!(observed, expected);
    }

    #[test]
    fn expr_btree() {
        let tree = Expr::Union(
            Box::new(Expr::Concat(
                Box::new(Expr::Union(
                    Box::new(Expr::Term('c')),
                    Box::new(Expr::Term('b')),
                )),
                Box::new(Expr::Kleene(Box::new(Expr::Union(
                    Box::new(Expr::Term('a')),
                    Box::new(Expr::Term('t')),
                )))),
            )),
            Box::new(Expr::Concat(
                Box::new(Expr::Term('c')),
                Box::new(Expr::Term('b')),
            )),
        );
        let expected = "c|b·a|t*|c·b";
        assert_eq!(format!("{}", tree), expected);
    }
    #[test_case("c|ba|t*|cb", false; "trailing char")]
    #[test_case("(c|ba)*a*", true; "complex kleene")]
    #[test_case("a*", true; "simple kleene")]
    #[test_case("", true; "empty string")]
    fn contains_epsilon(s: &str, expected: bool) {
        let expr: Expr = s.parse().unwrap();
        let result = expr.contains_epsilon();
        assert_eq!(result, expected);
    }

    #[test]
    fn expr_parse() {
        let s = "cb|a·t·".chars().collect::<Vec<_>>();
        let expr = Expr::parse_postfix(&s).unwrap();
        assert_eq!(format!("{}", expr), "c|b·a·t");
        assert_eq!(
            expr,
            Expr::Concat(
                Box::new(Expr::Concat(
                    Box::new(Expr::Union(
                        Box::new(Expr::Term('c')),
                        Box::new(Expr::Term('b'))
                    )),
                    Box::new(Expr::Term('a')),
                )),
                Box::new(Expr::Term('t'))
            )
        );
        assert!(!expr.contains_epsilon());
    }

    #[test]
    fn expr_deriv() {
        let s = "(c|b)at";
        let expr: Expr = s.parse().unwrap();
        let wrt_c = expr.derivative('c');
        assert_eq!(format!("{}", wrt_c.simplify_to_end()), "a·t");
    }

    #[test_case("", "", true; "empty string")]
    #[test_case("(c|b)at", "cat", true; "simple cat")]
    #[test_case("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)", "abcdefghijkl", true; "single")]
    fn is_match(regex: &str, pattern: &str, expected: bool) {
        let expr: Expr = regex.parse().unwrap();
        let observed = expr.is_match(pattern);
        assert_eq!(observed, expected);

        let compiled = regex::Regex::new(regex).unwrap();
        let according_to_regex = compiled.is_match(pattern);
        assert_eq!(observed, according_to_regex);
    }
}

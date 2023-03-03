# Brzozowski Derivative

*This code is provided for educational purpuses. It is not efficient. But it illustrates the idea.*

[Brzozowski derivative](https://en.wikipedia.org/wiki/Brzozowski_derivative) is a less known technique to work with regular expressions. Normally, to match a string using regex we have to construct an [automaton](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton) and simulate it. With the regex derivative technique we can use a regular expression 'directly' without the automaton construction and simulation.

It has nothing to do with classical derivative in analysis. But the symbolic nature and chain rule application make it feel similar. For more details there is a paper ["Regular-expression derivative re-examined"](https://www.ccs.neu.edu/home/turon/re-deriv.pdf) by Scott Owens, John Reppy and Aaron Turon.

## Definition and rules

The derivative of a language $L \subset \Sigma*$ with respect to a string $u \in \Sigma*$ is a language $\partial_u L = \lbrace v \mid u \cdot v \in L \rbrace$.

For any characters *a* and *b* and for any strings *r* and *s* we have following rules:

$$
\begin{align}
\partial_a \varepsilon &= \emptyset & \\
\partial_a a &= \epsilon & \\
\partial_a b &= \emptyset & \text{ for }(a \neq b) \\
\partial_a (r \cdot s) &= \partial_a r \mid \nu(r) \cdot \partial_a s & \\
\partial_a (r \mid s) &= \partial_a r \mid \partial_a s & \\
\partial_a (r*) &= \partial_a r \cdot r* &
\end{align}
$$

## Definition and rules

## Code and usage

This code implements only three operators. There are concatenation ($\cdot$), summation ('|'), and Kleene star(*).

The usage is straighforward:

```bash
> python match.py '(c|b)at' 'cat'
(((c|b)·a)·t)
(((ϵ|∅)·a)·t)
((((∅|∅)·a)|ϵ)·t)
(((((∅|∅)·a)|∅)·t)|ϵ)
True
```

```bash
> python match.py '(c|b)at' 'sat'
(((c|b)·a)·t)
(((∅|∅)·a)·t)
(((∅|∅)·a)·t)
(((∅|∅)·a)·t)
False
```

## Why do we need it

1. It is fun
2. Using this technique we can construct a very efficient automaton called [minimal DFA](https://en.wikipedia.org/wiki/DFA_minimization). There are some considerations though but in some cases such construction can be very handy.

## TODO

1. Extend the derivative theory section
2. Add the [DFA](https://en.wikipedia.org/wiki/Deterministic_finite_automaton) construction code

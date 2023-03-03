# Brzozowski Deriviative

(Brzozowski derivative)[https://en.wikipedia.org/wiki/Brzozowski_derivative] is a less known technique to work with regular expression. Normally, to match a string using regex we have to construct an (automaton)[https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton] and simulate it. With the regex derivative technique we can use a regular expression 'directly' without the automaton construction and simulation.

It has nothing to do with classical derivative in analysis. But the symbolic nature and chain rule application make it feel similar.
For more details there is a paper "Regular-expression derivative re-examined" by Scott Owens, John Reppy and Aaron Turon.

Below is the example of a derivative $\partial_{cat}(c|b)a*t$:

```
(c|m)·a·t
(ϵ|∅)·a·t
(((∅|∅)·a)|ϵ)·t
(((∅|∅)·a)|∅)·t|ϵ
```


## Why do we need it

1. It is fun
2. Using this technique we can construct a very efficient automaton called (minimal DFA)[https://en.wikipedia.org/wiki/DFA_minimization]. There are some considerations though but in some cases such construction can be very handy.

## TODO

1. Extend the derivative theory section
2. Add the *DFA* construction code

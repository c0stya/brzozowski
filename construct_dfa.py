import sys
from collections import defaultdict
from deriv import deriv, norm, nullable, infix_to_btree, btree_to_infix

# from match_dfa import normalize

""" Algorithm schema:

Q <- r                  queue to keep unexplored states
D <- r                  hash table to store already explored states
while Q is not empty
    r <- Q.pop()

    for any c in alphabet A:
        s = take a derivative of r wrt c
        s = normalize s

        if s not in dictionary D, then
            Q.push(s)
"""


def construct_dfa(r, A):
    Q = []
    D = {}
    finals = set()

    # append r
    start = norm(r)
    D[start] = {}
    Q.append(start)

    if nullable(start) == "ε":
        finals.add(start)

    while Q:
        r = Q.pop(0)

        for c in A:
            s = deriv(r, c)
            s = norm(s)

            D[r][c] = s

            if s not in D:  # new state s
                Q.append(s)
                D[s] = {}

                if nullable(s) == "ε":
                    finals.add(s)

    return D, start, finals


def dict_to_graphviz(dfa):
    arcs = []
    D, start, finals = dfa

    for src, transitions in D.items():
        src_ = btree_to_infix(src)
        if src in finals:
            arcs.append(f'"{src_}" [peripheries=2];')
        else:
            arcs.append(f'"{src_}";')

        # merge labels
        dst_dict = defaultdict(lambda: [])
        for label, dst in transitions.items():
            dst_dict[dst].append(label)
        for dst, labels in dst_dict.items():
            dst_ = btree_to_infix(dst)
            arcs.append(
                f'"{src_}" -> "{dst_}" [ label="{",".join(sorted(labels))}" ];'
            )

    return "digraph G {\n\tsplines=true; rankdir=LR;\n\t" + "\n\t".join(arcs) + "\n}"


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: {} <regexp>".format(sys.argv[0]))
        sys.exit(1)

    # alphabet
    regex = sys.argv[1]

    # infer the alphabet
    A = sorted([c for c in set(regex) if c not in "()|*"])

    dfa = construct_dfa(infix_to_btree(regex), A)
    print(dict_to_graphviz(dfa))

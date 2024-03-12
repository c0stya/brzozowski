import sys
from deriv import deriv, norm, infix_to_btree, btree_to_infix, nullable


def match(regex, string, show_inference=True):
    q = infix_to_btree(regex)

    if show_inference:
        print(btree_to_infix(q))

    for c in string:
        q = deriv(q, c)
        q = norm(q)

        if show_inference:
            print("{}: {}".format(c, btree_to_infix(q)))

    return nullable(q) == "ε"


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: {} <regexp> <string>".format(sys.argv[0]))
        sys.exit(1)

    print(match(sys.argv[1], sys.argv[2], show_inference=True))

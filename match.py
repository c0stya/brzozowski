import sys

prec = {'(': 0, '|': 1, '·': 2, '*': 3}


def augment(src):
    dst = []
    for i in range(len(src)):
        if i > 0 and not (src[i] in '|)*' or src[i - 1] in '(|'):
            dst.append('·')
        dst.append(src[i])

    return ''.join(dst)


def inorder(root):
    if root is None:
        return
    if root.c in '|·*':
        print('(', end='')
    inorder(root.l)
    print(root.c, end='')
    inorder(root.r)
    if root.c in '|·*':
        print(')', end='')


def infix_to_postfix(exp):
    stack = []
    output = []

    for c in exp:
        if c.isalpha():
            output.append(c)
        elif c == "(":
            stack.append(c)
        elif c == ")":
            while len(stack) > 0 and stack[-1] != "(":
                output.append(stack.pop())
            else:
                stack.pop()
        else:
            while len(stack) > 0 and prec[stack[-1]] >= prec[c]:
                output.append(stack.pop())
            stack.append(c)

    while len(stack) > 0:
        output.append(stack.pop())

    return "".join(output)


class Node:
    def __init__(self, data, left=None, right=None):
        self.c = data
        self.l = left
        self.r = right


def postfix_to_tree(postfix):
    if not postfix:
        return

    stack = []

    for c in postfix:
        if c in "|·":
            r, l = stack.pop(), stack.pop()
            stack.append(Node(c, l, r))
        elif c in "*":
            l = stack.pop()
            stack.append(Node(c, l))
        else:
            stack.append(Node(c))

    return stack[-1]


def clone(node):
    if node is None:
        return None
    return Node(node.c, clone(node.l), clone(node.r))


def nullable(node):
    if node is None:
        return False
    elif node.c == 'ϵ' or node.c == '*':
        return True
    elif node.c == '·':
        return nullable(node.l) and nullable(node.r)
    elif node.c == '|':
        return nullable(node.l) or nullable(node.r)
    else:
        return False


def deriv(root, c):
    stack = [root]

    while len(stack) > 0:
        node = stack.pop()
        if node is None or node.c == '∅':   # ∂ₐ(∅)     = ∅
            continue
        elif node.c == 'ϵ':                 # ∂ₐ(ϵ)     = ∅
            node.c = '∅'
        elif node.c == c:                   # ∂ₐ(a)     = ϵ
            node.c = 'ϵ'
        elif node.c == "|":                 # ∂ₐ(r|s)   = ∂ₐ(r) | ∂ₐ(s)
            stack.append(node.l)
            stack.append(node.r)
        elif node.c == "·":                 # ∂ₐ(rs)    = ∂ₐ(r) | nullable(r) ∂ₐ(s)
            if nullable(node.l):
                node.c = "|"
                dnode = Node("·", node.l, node.r)
                node.l = dnode
                node.r = clone(dnode.r)
                stack.append(node.l.l)
                stack.append(node.r)
            else:
                stack.append(node.l)
        elif node.c == "*":                 # ∂ₐ(r*)    = ∂ₐ(r) r*
            star_node = clone(node)
            node.c = "·"
            node.r = star_node
            stack.append(node.l)
        else:
            node.c = '∅'                    # ∂ₐ(r*)    = ∅ (if a ≠ b)

    return root


def match(node, string):
    inorder(node); print()
    for c in string:
        deriv(node, c)
        inorder(node); print()
    return nullable(node)


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print("Usage: {} regexp string".format(sys.argv[0]))
        sys.exit(1)

    root = postfix_to_tree(infix_to_postfix(augment(sys.argv[1])))
    print(match(root, sys.argv[2]))

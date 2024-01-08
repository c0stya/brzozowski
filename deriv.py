prec = {'(': 0, '|': 1, '·': 2, '*': 3}

def augment(src):
    if not src:
        return 'ε'
    dst = []
    for i in range(len(src)):
        if i > 0 and not (src[i] in '|)*' or src[i - 1] in '(|'):
            dst.append('·')
        dst.append(src[i])

    return ''.join(dst)


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


def postfix_to_btree(postfix):
    if not postfix:
        return

    stack = []
    for c in postfix:
        if c == '|':
            r, l = stack.pop(), stack.pop()
            stack.append((c, l, r))
        elif c == '·':
            r, l = stack.pop(), stack.pop()
            stack.append((c, l, r))
        elif c in "*":
            l = stack.pop()
            stack.append((c, l))
        else: # operand
            stack.append(c)

    return stack[-1]


# helper: input string -> tree structure
def infix_to_btree(regex):
    return  postfix_to_btree(
            infix_to_postfix(augment(regex)))


def nullable(q):
    if q == 'ε':
        return 'ε'
    elif len(q) == 1:
        return '∅'

    if len(q) == 3:
        op, r, s = q
    else:
        op, r = q

    if op == '·':
        if nullable(r) == 'ε' and nullable(s) == 'ε':
            return 'ε'
    elif op == '|':
        if nullable(r) == 'ε' or nullable(s) == 'ε':
            return 'ε'
    else:                           # op == '*'
        return 'ε'

    return '∅'


def deriv(q, c):
    if q == '∅':                    # ∂ₐ(∅)     = ∅
        return '∅'
    if q == 'ε':                    # ∂ₐ(ε)     = ∅
        return '∅'
    if q == c:                      # ∂ₐ(a)     = ε
        return 'ε'
    if len(q) == 1 and q != c:      # ∂ₐ(b)     = ∅  (a ≠ b)
        return '∅'

    if len(q) == 3:
        op, r, s = q
    else:
        op, r = q

    if op == "|":                   # ∂ₐ(r|s)   = ∂ₐ(r) | ∂ₐ(s)
        return ('|',
                deriv(r, c),
                deriv(s, c)
            )
    elif op == '·':                 # ∂ₐ(rs)    = ∂ₐ(r) s | ν(r) ∂ₐ(s)
        return ('|',
                ('·', deriv(r, c), s),
                ('·', nullable(r), deriv(s, c))
            )
    elif op == "*":                 # ∂ₐ(r*)    = ∂ₐ(r) r*
        return ('·',
                deriv(r, c),
                ('*', r)
            )
    else:
        raise ValueError(r'Unsupported op: {op}')



def _norm(q):
    if len(q) == 1:
        return q
    elif len(q) == 2:
        op, r = q
        r_ = _norm(r)
    else:
        op, r, s = q
        r_, s_ = _norm(r), _norm(s)

    if op == '|':
        if r_ == '∅':
            return s_                               # ∅|a = a
        elif s_ == '∅':                             # a|∅ = a
            return r_
    elif op == '·':
        if r_ == '∅' or s_ == '∅':                  # ∅·a = a·∅ = ∅
            return '∅'
        elif r_ == 'ε':                             # ε·a = a
            return s_
        elif s_ == 'ε':                             # a·ε = a
            return r_
    elif op == '*':
        if len(r_) == 2:                            # (a*)* = a*
            return r_
        elif r_ == 'ε' or r_ == '∅':                # ε* = ε, ∅* = ε
            return 'ε'
        else:
            return (op, r_)

    return (op, r_, s_)


def traverse(r):
    if len(r) == 1:
        yield r
    else:
        op, s = r
        yield op
        for t in s:
            for tt in traverse(t):
                yield tt
        yield '$'


def compare(r, s):
    itr = traverse(r)
    its = traverse(s)

    for tr, ts in zip(itr, its):
        if tr < ts:
            return -1
        elif tr > ts:
            return 1

    tr = next(itr, None)
    ts = next(its, None)

    if not (tr or ts):
        return 0

    return -1 if tr else 1


def merge_sort(l, r):
    merged = []
    i, j = 0, 0

    while i < len(l) and j < len(r):
        cmp = compare(l[i], r[j])
        if cmp < 0:
            merged.append(l[i])
            i += 1
        elif cmp > 0:
            merged.append(r[j])
            j += 1
        else:
            merged.append(l[i])
            i += 1
            j += 1

    merged.extend(l[i:] + r[j:])

    return tuple(merged)


def _sort(q):
    '''Unfold and sort'''
    if len(q) == 1:
        return q
    if len(q) == 2:
        op, r = q
        return (op, _sort(r))
    else:
        op, r, s = q
        r, s = _sort(r), _sort(s)
        r = r[1] if op == r[0] else (r,)
        s = s[1] if op == s[0] else (s,)
        return (op, merge_sort(r, s)) if op == '|' else (op, r + s)


def mtree_to_btree(r):
    if len(r) == 1:                     # reached a character
        return r

    op, s = r
    if op == '*':
        return (op, mtree_to_btree(s))
    elif len(s) == 1:                   # pathalogical node of (|,(r,))
        return mtree_to_btree(s[0])     # needs to be unpacked
    elif len(s) == 2:                   # two final items in a tree
        return (op, mtree_to_btree(s[0]),  mtree_to_btree(s[1]))
    else:
        return (op, mtree_to_btree(s[0]), mtree_to_btree((op, s[1:])))


def norm(q):
    return mtree_to_btree(_sort(_norm(q)))


def btree_to_infix(q):
    def _traverse(q):
        if len(q) == 1:
            return None, q
        elif len(q) == 2:
            op, r = q
            pop, pr = _traverse(r)
            pr = '(' + pr + ')' if pop else pr
            return op, pr + op
        else:
            op, r, s = q
            popr, pr = _traverse(r)
            pops, ps = _traverse(s)
            if popr and prec[popr] < prec[op]:
                pr = '(' + pr + ')'
            if pops and prec[pops] < prec[op]:
                ps = '(' + ps + ')'
            return op, pr + op + ps
    pop, r = _traverse(q)
    return r


if __name__ == '__main__':
    s = '(a|(ab)*)|(abcde)*'

    ps = infix_to_postfix(augment(s))
    btree = postfix_to_btree(ps)
    print (btree)
    btree = btree_to_infix(btree)
    print (btree)

    #print (deriv_norm(btree, 'a'))

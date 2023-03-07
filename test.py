from match import match

samples = [
    ('', '', True),
    ('', 'a', False),
    ('a', '', False),
    ('a', 'b', False),
    ('a', 'a', True),
    ('a|b', '', False),
    ('a|b', 'a', True),
    ('a|b', 'c', False),
    ('a*', '', True),
    ('a*', 'aaa', True),
    ('a*', 'b', False),
    ('a*b*', '', True),
    ('a*b*', 'aabb', True),
    ('a*b*', 'aabba', False),
    ('(a*)*', '', True),
    ('(a*)*', 'aa', True),
    ('(a|b)*', '', True),
    ('(a|b)*', 'ab', True),
    ('(a|b)*', 'aa', True),
    ('(a|b)*', 'bb', True),
    ('(a|b)*', 'abab', True),
]

for regex, string, out in samples:
    assert out == match(regex, string), \
        "{}\t{}\texpected: {}".format(regex, string, out)

print('We are good')

entities = [
    ('Alpha', 913),
    ('Beta', 914),
    ('Chi', 935),
    ('Delta', 916),
    ('Epsilon', 917),
    ('Eta', 919),
    ('Gamma', 915),
    ('Iota', 921),
    ('Kappa', 922),
    ('Lambda', 923),
    ('Mu', 924),
    ('Nu', 925),
    ('Omega', 937),
    ('Omicron', 927),
    ('Phi', 934),
    ('Pi', 928),
    ('Psi', 936),
    ('Rho', 929),
    ('Sigma', 931),
    ('Tau', 932),
    ('Theta', 920),
    ('Upsilon', 933),
    ('Xi', 926),
    ('Zeta', 918),
    ('alpha', 945),
    ('and', 8743),
    ('asymp', 8776),
    ('because', 8757),
    ('beta', 946),
    ('bullet', 8729),
    ('cap', 8745),
    ('chi', 967),
    ('circ', 8728),
    ('cup', 8746),
    ('delta', 948),
    ('dot', 8901),
    ('downarrow', 8595),
    ('empty', 8709),
    ('epsilon', 949),
    ('equiv', 8801),
    ('eta', 951),
    ('exist', 8707),
    ('forall', 8704),
    ('gamma', 947),
    ('ge', 8805),
    ('geq', 8805),
    ('ggt', 8811),
    ('gt', 62),
    ('in', 8712),
    ('inf', 8734),
    ('infin', 8734),
    ('infty', 8734),
    ('iota', 953),
    ('kappa', 954),
    ('lambda', 955),
    ('lcb', 123),
    ('le', 8804),
    ('leftarrow', 8592),
    ('leq', 8804),
    ('llt', 8810),
    ('lt', 60),
    ('mp', 8723),
    ('mu', 956),
    ('nabla', 8711),
    ('ne', 8800),
    ('neq', 8800),
    ('nequiv', 8802),
    ('ni', 8715),
    ('notin', 8713),
    ('notni', 8716),
    ('nsub', 8836),
    ('nsube', 8840),
    ('nsup', 8837),
    ('nsupe', 8841),
    ('nu', 957),
    ('null', 8709),
    ('odiv', 8856),
    ('odot', 8857),
    ('omega', 969),
    ('omicron', 959),
    ('ominus', 8854),
    ('oplus', 8853),
    ('or', 8744),
    ('otimes', 8855),
    ('partial', 8706),
    ('phi', 966),
    ('pi', 960),
    ('pm', 177),
    ('prop', 8733),
    ('psi', 968),
    ('qed', 8718),
    ('rcb', 125),
    ('rho', 961),
    ('rightarrow', 8594),
    ('sigma', 963),
    ('simeq', 8771),
    ('star', 8902),
    ('sub', 8834),
    ('sube', 8838),
    ('sup', 8835),
    ('supe', 8839),
    ('tau', 964),
    ('therefore', 8756),
    ('theta', 952),
    ('times', 215),
    ('triangle', 8710),
    ('uparrow', 8593),
    ('upsilon', 965),
    ('xi', 958),
    ('zeta', 950),
]

def strip_prefix(entities):
    c = {}

    for e in entities:
        prefix = e[0][0] if len(e[0]) > 0 else None

        if prefix in c:
            c[prefix].append((e[0][1:], e[1]))

        else:
            c[prefix] = [(e[0][1:], e[1])]

    for p in c.keys():

        if len(c[p]) > 1:
            c[p] = strip_prefix(c[p])

    return c

c = strip_prefix(entities)

def print_branch(tree, indent, index):
    if len(tree) == 1:
        print_branch(list(tree.values())[0], indent, index + 1)
        return

    indent_string = " " * (indent * 4)

    if None in tree:
        print(f"{indent_string}if word.len() == {index} {'{'} {wrap_num(tree[None][0][1])} {'}'}\n")

    for ind, p in enumerate(tree.keys()):

        if p is None:
            continue

        if ind == 0:
            cond = f"if word[{index}] == '{p}' as u32"

        elif ind == len(tree) - 1:
            cond = "else"

        else:
            cond = f"else if word[{index}] == '{p}' as u32"

        if type(tree[p]) is list:
            print(f"{indent_string}{cond} {'{'} {wrap_num(tree[p][0][1])} {'}'}\n")

        else:
            assert type(tree[p]) is dict
            print(f"{indent_string}{cond} {'{'}\n")
            print_branch(tree[p], indent + 1, index + 1)
            print(indent_string + "}\n")

def wrap_num(n):
    return f"Entity::new_character({n})"

print_branch(c, 0, 0)
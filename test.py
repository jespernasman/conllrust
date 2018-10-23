
with open('example.txt') as f:
    example = f.readlines()

from conllrust import frequencies, nq

class Token:
    form = None

tokens = []
for x in range(10):
    t = Token()
    t.form = str(x)
    tokens.append(t)

nq(tokens[0])

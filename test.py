
with open('example.txt') as f:
    example = f.readlines()

from conllrust import frequencies

print(frequencies(example, 5, ['VERB', 'ADV'], 2))

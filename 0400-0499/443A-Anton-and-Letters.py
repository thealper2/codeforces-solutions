word = input().strip()
word = word[1:-1].split(',')
word = [w.strip() for w in word if w != '']
print(len(set(word)))

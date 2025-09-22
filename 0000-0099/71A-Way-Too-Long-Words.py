n = int(input())
words = []
for _ in range(n):
	word = str(input())
	words.append(word)

for word in words:
	l = len(word)
	if l <= 10:
		print(word)
	else:
		print(word[0] + str(l - 2) + word[-1])

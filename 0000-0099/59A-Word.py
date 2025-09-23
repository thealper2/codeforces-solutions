word = input().strip()
n = 0
for c in word:
	if c.isupper():
		n += 1
	else:
		n -= 1

if n <= 0:
	print(word.lower())
else:
	print(word.upper())

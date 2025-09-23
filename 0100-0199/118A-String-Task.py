word = input().strip().lower()
vowels = 'aeiouy'
result = ''
for c in word:
	if c not in vowels:
		result += '.'
		result += c

print(result)

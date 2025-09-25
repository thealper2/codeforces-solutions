n = int(input())
freq = {}

result = []
for _ in range(n):
	s = input()
	if s not in freq:
		result.append('OK')
		freq[s] = 1
	else:
		result.append(s + str(freq.get(s)))
		freq[s] += 1

for r in result:
	print(r)

n = int(input())
for _ in range(n):
	s = input()
	result = sum(1 for a, b in zip(s, 'codeforces') if a != b)
	print(result)

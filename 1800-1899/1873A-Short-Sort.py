n = int(input())
for _ in range(n):
	s = input()
	diff = sum(1 for a, b in zip(s, 'abc') if a != b)
	if diff > 2:
		print('NO')
	else:
		print('YES')

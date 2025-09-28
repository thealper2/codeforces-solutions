n = int(input())
chars = set('codeforces')
for _ in range(n):
	c = input().strip()
	if c in chars:
		print('YES')
	else:
		print('NO')

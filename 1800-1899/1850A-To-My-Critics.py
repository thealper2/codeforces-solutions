n = int(input())
for _ in range(n):
	a, b, c = map(int, input().split())
	c1 = a + b >= 10
	c2 = a + c >= 10
	c3 = b + c >= 10
	if c1 or c2 or c3:
		print('YES')
	else:
		print('NO')

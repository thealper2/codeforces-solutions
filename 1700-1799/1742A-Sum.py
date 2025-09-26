n = int(input())
for _ in range(n):
	a, b, c = map(int, input().split())
	points = sorted([a, b, c])
	if points[2] - points[1] == points[0]:
		print('YES')
	else:
		print('NO')

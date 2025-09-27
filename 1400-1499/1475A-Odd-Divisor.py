n = int(input())
for _ in range(n):
	value = int(input())
	if value & (value - 1) == 0:
		print('NO')
	else:
		print('YES')

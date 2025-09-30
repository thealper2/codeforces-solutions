t = int(input())
for _ in range(t):
	n = int(input())
	arr = list(map(int, input().split()))
	if len(arr) == 1:
		print('NO')
	else:
		odds = sum(x % 2 for x in arr)
		if odds % 2 == 0:
			print('YES')
		else:
			print('NO')

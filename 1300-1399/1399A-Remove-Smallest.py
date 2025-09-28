n = int(input())
for _ in range(n):
	_ = int(input())
	arr = list(map(int, input().split()))
	while len(arr) > 1:
		arr.sort()
		found = False
		for i in range(len(arr) - 1):
			if abs(arr[i] - arr[i + 1]) <= 1:
				arr.pop(i)
				found = True
				break

		if not found:
			break

	if len(arr) == 1:
		print('YES')
	else:
		print('NO')

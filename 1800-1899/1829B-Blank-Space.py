t = int(input())
for _ in range(t):
	n = int(input())
	arr = list(map(int, input().split()))
	zc = 0
	max_zc = 0
	for i in range(n):
		if arr[i] == 0:
			zc += 1
		else:
			max_zc = max(max_zc, zc)
			zc = 0

	max_zc = max(max_zc, zc)
	print(max_zc)

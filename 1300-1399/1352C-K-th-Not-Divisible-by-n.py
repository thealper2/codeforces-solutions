t = int(input())
for _ in range(t):
	n, k = map(int, input().split())
	result = k + (k - 1) // (n - 1)
	print(result)

n = int(input())
arr = [0, 0, 0]
for _ in range(n):
	a, b, c = map(int, input().split())
	arr[0] -= a
	arr[1] -= b
	arr[2] -= c

if all(item == 0 for item in arr):
	print('YES')
else:
	print('NO')

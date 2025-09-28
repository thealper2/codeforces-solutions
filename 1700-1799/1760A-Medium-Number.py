n = int(input())

for _ in range(n):
	arr = list(map(int, input().split()))
	first = float('-inf')
	second = float('-inf')
	for item in arr:
		if item > first:
			second = first
			first = item
		elif item > second:
			second = item


	print(second)

n, t = map(int, input().split())
arr = list(map(int, input().split()))

max_books = 0
current_sum = 0
l = 0

for r in range(n):
	current_sum += arr[r]
	while current_sum > t:
		current_sum -= arr[l]
		l += 1

	max_books = max(max_books, r - l + 1)

print(max_books)

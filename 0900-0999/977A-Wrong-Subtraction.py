number, k = map(int, input().split())

for _ in range(k):
	d = number % 10
	if d == 0:
		number //= 10
	else:
		number -= 1

print(number)

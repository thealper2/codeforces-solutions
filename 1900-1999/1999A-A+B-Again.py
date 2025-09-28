n = int(input())

for _ in range(n):
	number = int(input())
	total = 0
	i = 0
	while 10**i <= number:
		d = number // 10**i % 10
		total += d
		i += 1

	print(total)

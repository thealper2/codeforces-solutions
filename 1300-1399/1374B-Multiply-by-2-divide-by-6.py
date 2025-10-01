t = int(input())
for _ in range(t):
	n = int(input())
	cnt = 0
	while n % 3 == 0:
		if n % 6 == 0:
			n //= 6
		else:
			n *= 2

		cnt += 1

	print(cnt if n == 1 else -1)

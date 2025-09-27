n = int(input())
for _ in range(n):
	value = int(input())
	sum_ = 0
	i = 0
	while 10**i <= value:
		d = value // 10**i % 10
		if i < 3:
			sum_ += d
		else:
			sum_ -= d

		i += 1

	if sum_ == 0:
		print('YES')
	else:
		print('NO')

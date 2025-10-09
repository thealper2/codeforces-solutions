t = int(input())
for _ in range(t):
	n = int(input())
	k = n // 2
	if k % 2 == 1:
		print('NO')
	else:
		print('YES')
		first_half = [2 * i for i in range(1, k + 1)]
		second_half = [2 * i - 1 for i in range(1, k)]
		second_half.append(3 * k - 1)
		print(' '.join(map(str, first_half + second_half)))

t = int(input())
for _ in range(t):
	n = int(input())
	half = n // 2
	if half % 2 == 1:
		print('NO')
	else:
		print('YES')
		evens = [2 * i for i in range(1, half + 1)]
		odds = [2 * i - 1 for i in range(1, half)]
		last_odd = sum(evens) - sum(odds)
		odds.append(last_odd)
		result = evens + odds
		print(' '.join(map(str, result)))

n = int(input())
for _ in range(n):
	number = input().strip()
	result = []
	l = len(number)
	for i, c in enumerate(number):
		if c != '0':
			result.append(int(c) * 10 ** (l - i - 1))

	print(len(result))
	print(*result)

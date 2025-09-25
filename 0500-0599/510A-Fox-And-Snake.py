n, m = map(int, input().split())
is_left = True
for i in range(n):
	if i % 2 == 0:
		print('#' * m)
	else:
		if is_left:
			print('.' * (m - 1) + '#')
		else:
			print('#' + '.' * (m - 1))

		is_left = not is_left

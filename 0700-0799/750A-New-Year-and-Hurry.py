n, k = map(int, input().split())
available = 240 - k

solved = 0
time_used = 0

for i in range(1, n + 1):
	time_used += 5 * i
	if time_used > available:
		break

	solved += 1

print(solved)

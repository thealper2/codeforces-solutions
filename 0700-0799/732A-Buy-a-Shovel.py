k, r = map(int, input().split())

for need in range(1, 11):
	if (k * need) % 10 == 0 or (k * need) % 10 == r:
		print(need)
		break

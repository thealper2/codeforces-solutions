n = int(input())
for _ in range(n):
	a, b, c = map(int, input().split())
	if a < b < c:
		print('STAIR')
	elif a < b and c < b:
		print('PEAK')
	else:
		print('NONE')


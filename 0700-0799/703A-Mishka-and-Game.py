n = int(input())
cnt = 0
for _ in range(n):
	a, b = map(int, input().split())
	if a > b:
		cnt += 1
	elif b > a:
		cnt -= 1

if cnt > 0:
	print('Mishka')
elif cnt < 0:
	print('Chris')
else:
	print('Friendship is magic!^^')

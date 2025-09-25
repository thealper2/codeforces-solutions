n = int(input())
s = input().lower()

if len(s) <= 25:
	print('NO')
else:
	if len(set(s)) <= 25:
		print('NO')
	else:
		print('YES')

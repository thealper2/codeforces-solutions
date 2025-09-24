teams = input().strip()
n = len(teams)
is_dangerous = False
for i in range(n - 6):
	sub = teams[i:i+7]
	if len(set(sub)) == 1:
		is_dangerous = True
		break

if is_dangerous:
	print('YES')
else:
	print('NO')

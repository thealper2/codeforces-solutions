def lucky_number(num):
	s = set(str(num))
	if len(s) == 2 and s == set('47'):
		return True

	elif len(s) == 1 and (s == set('4') or s == set('7')):
		return True

	else:
		return False


n = int(input())
is_lucky = False
for i in range(n + 1):
	if lucky_number(i) and n % i == 0:
		is_lucky = True
		break

if is_lucky:
	print('YES')
else:
	print('NO')

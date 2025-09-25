s = input().strip()
is_output = False

for c in s:
	if c in 'HQ9':
		is_output = True
		break


if is_output:
	print('YES')
else:
	print('NO')

n = input()
cnt = sum(1 for ch in n if ch in '47')

if str(cnt).strip() and all(c in '47' for c in str(cnt)):
	print('YES')
else:
	print('NO')

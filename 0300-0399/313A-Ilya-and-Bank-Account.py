n = input().strip()

if n[0] != '-':
	print(n)
else:
	opt1 = n[:-1]
	opt2 = n[:-2] + n[-1]
	num1 = int(opt1)
	num2 = int(opt2)
	print(max(num1, num2))

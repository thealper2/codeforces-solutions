s = input().strip()
if len(s) == 1 and s[0].islower():
	print(s.upper())
elif s.isupper() or (s[1:].isupper() and s[0].islower()):
	print(s.swapcase())
else:
	print(s)

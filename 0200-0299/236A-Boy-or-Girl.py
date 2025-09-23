username = input()
unique = set(username)
n = len(unique)
if n % 2 == 1:
	print('IGNORE HIM!')
else:
	print('CHAT WITH HER!')

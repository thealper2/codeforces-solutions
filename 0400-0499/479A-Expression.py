a = int(input())
b = int(input())
c = int(input())

vals = [
	a + b + c,
	a * b * c,
	(a + b) * c,
	a * (b + c),
	a + (b * c),
	(a * b) + c,
	(a * b) * c,
	a * (b * c)
]

print(max(vals))

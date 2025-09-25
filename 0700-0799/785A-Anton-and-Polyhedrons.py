n = int(input())
d = {
	'Tetrahedron': 4,
	'Cube': 6,
	'Octahedron': 8,
	'Dodecahedron': 12,
	'Icosahedron': 20
}

result = 0
for _ in range(n):
	f = input().strip()
	result += d.get(f, 0)

print(result)

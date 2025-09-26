import math

n = int(input())
arr = list(map(int, input().split()))

limit = 10**6 + 1
is_prime = [True] * limit
is_prime[0] = is_prime[1] = False
for i in range(2, int(limit**0.5) + 1):
	if is_prime[i]:
		for j in range(i * i, limit, i):
			is_prime[j] = False

primes = set(i for i, v in enumerate(is_prime) if v)
for x in arr:
	root = int(math.isqrt(x))
	if root * root == x and root in primes:
		print('YES')
	else:
		print('NO')

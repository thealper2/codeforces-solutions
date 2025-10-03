n, m = map(int, input().split())
def is_prime(num):
	if num < 2:
		return False

	for i in range(2, int(num**0.5) + 1):
		if num % i == 0:
			return False

	return True

primes = [num for num in range(2, 51) if is_prime(num)]
idx_n = primes.index(n)
if idx_n + 1 < len(primes) and primes[idx_n + 1] == m:
	print('YES')
else:
	print('NO')

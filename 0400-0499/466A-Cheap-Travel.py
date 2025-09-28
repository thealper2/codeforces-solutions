n, m, a, b = map(int, input().split())
cost_with_packages = (n // m) * b + min((n % m) * a, b)
cost_with_single = n * a
print(min(cost_with_packages, cost_with_single))

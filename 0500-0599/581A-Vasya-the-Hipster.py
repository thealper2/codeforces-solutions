a, b = map(int, input().split())
min_ = min(a, b)
a -= min_
b -= min_
max_ = max(a // 2, b // 2)
print(min_, max_)

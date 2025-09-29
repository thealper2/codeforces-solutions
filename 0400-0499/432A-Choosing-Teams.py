n, k =map(int, input().split())
y = list(map(int, input().split()))
eligible = sum(1 for val in y if val + k <= 5)
print(eligible // 3)

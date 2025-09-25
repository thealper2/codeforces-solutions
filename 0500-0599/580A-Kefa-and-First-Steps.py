n = int(input())
steps = list(map(int, input().split()))
max_l = 1
l = 1
prev = steps[0]

for i in range(1, n):
        if steps[i] >= steps[i - 1]:
                l += 1
        else:
                l = 1

        max_l = max(max_l, l)

print(max_l)

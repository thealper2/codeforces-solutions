import math

n, m, a = map(int, input().split())
stones_length = math.ceil(n / a)
stones_width = math.ceil(m / a)
total_stones = stones_length * stones_width
print(total_stones)

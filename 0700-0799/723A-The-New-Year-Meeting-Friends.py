x1, x2, x3  = map(int, input().split())
points = sorted([x1, x2, x3])
print(points[2] - points[0])

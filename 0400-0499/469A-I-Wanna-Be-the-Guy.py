n = int(input())
x = list(map(int, input().split()))[1:]
y = list(map(int, input().split()))[1:]
levels = set(x + y)
print("I become the guy." if len(levels) == n else "Oh, my keyboard!")

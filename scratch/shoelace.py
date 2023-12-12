from math import ceil, floor

def shoelace(verts):
    print(verts)
    """
    Calculate the area of a polygon defined by a list of vertices.
    """
    n = len(verts)
    a = 0
    for i in range(n):
        j = (i + 1) % n

        x1 = verts[i][0]
        x2 = verts[j][0]
        y1 = verts[i][1]
        y2 = verts[j][1]

        if x1 > x2:
            x1 = ceil(x1 - 0.5)
            x2 = floor(x2 +0.5)

        if x2 > x1:
            x2 = ceil(x2 - 0.5)
            x1 = floor(x1 +0.5)

        if y1 > y2:
            y1 = ceil(y1 - 0.5)
            y2 = floor(y2 +0.5)

        if y2 > y1:
            y2 = ceil(y2 - 0.5)
            y1 = floor(y1 +0.5)

        a += x1 * y2
        a -= x2 * y1
    return abs(a) / 2

print(shoelace([
    [1,1],
    [3,1],
    [3,3],
    [5,3],
    [5,1],
    [7,1],
    [7,5],
    [1,5],
]), 9)

print(shoelace([
    [1,1],
    [3,1],
    [3,3],
    [1,3],
]), 1)

print(shoelace([
    [1,1],
    [3,1],
    [3,3],
    [5,3],
    [5,1],
    [7,1],
    [7,6],
    [1,6],
]), 14)


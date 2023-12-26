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

        if x2 > x1:
            x2 += 1
        else: 
            x1 += 1

        if y2 > y1:
            y2 += 1
        else: 
            y1 += 1

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

print("day 18")
print(shoelace([
    [1, 0],
    [7, 0],
    [7, 5],
    [5, 5],
    [5, 7],
    [7, 7],
    [7, 9],
    [2, 9],
    [2, 7],
    [1, 7],
    [1, 5],
    [3, 5],
    [3, 2],
    [1, 2],
]), "==", 62)


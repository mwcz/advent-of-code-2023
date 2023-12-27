def shoelace(verts):
    """
    Calculate the area of a polygon defined by a list of vertices.
    """

    # find bounds
    min_x = min(verts, key=lambda v: v[0])[0]
    max_x = max(verts, key=lambda v: v[0])[0]
    min_y = min(verts, key=lambda v: v[1])[1]
    max_y = max(verts, key=lambda v: v[1])[1]
    width = max_x - min_x
    height = max_y - min_y
    x_bump = 1 / (width)
    y_bump = 1 / (height)
    bump = max(x_bump, y_bump)

    print("min_x", min_x)
    print("max_x", max_x)
    print("min_y", min_y)
    print("max_y", max_y)
    print("width", width)
    print("height", height)
    print("x_bump", x_bump)
    print("y_bump", y_bump)

    n = len(verts)
    a = 0

    # account for pixels having area by shifting each vertex outward
    for i in range(n):
        bumped = (verts[i][0] - min_x) * bump
        verts[i][0]  += bumped
        bumped = (verts[i][1] - min_y) * bump
        verts[i][1]  += bumped

    print()

    min_x = min(verts, key=lambda v: v[0])[0]
    max_x = max(verts, key=lambda v: v[0])[0]
    min_y = min(verts, key=lambda v: v[1])[1]
    max_y = max(verts, key=lambda v: v[1])[1]
    width = max_x - min_x
    height = max_y - min_y

    print("min_x", min_x)
    print("max_x", max_x)
    print("min_y", min_y)
    print("max_y", max_y)
    print("width", width)
    print("height", height)
    print()
    for vert in verts:
        print(vert)

    print()


    for i in range(n):
        j = (i + 1) % n

        x1 = verts[i][0]
        x2 = verts[j][0]
        y1 = verts[i][1]
        y2 = verts[j][1]

        a += x1 * y2
        a -= x2 * y1

    return abs(a) / 2

# print(shoelace([
#     [1,1],
#     [3,1],
#     [3,3],
#     [5,3],
#     [5,1],
#     [7,1],
#     [7,5],
#     [1,5],
# ]), 9)

# print(shoelace([
#     [1,1],
#     [5,1],
#     [5,5],
#     [1,5],
# ]))

# print(shoelace([
#     [1,6],
#     [7,6],
#     [7,1],
#     [5,1],
#     [5,3],
#     [3,3],
#     [3,1],
#     [1,1],
# ]), 40)

# print(shoelace([
#     [2,1],
#     [10,1],
#     [8,6],
#     [11,7],
#     [7,10],
# ]), 32.0)

print(shoelace([
    [0, 0],
    [6, 0],
    [6, 5],
    [4, 5],
    [4, 7],
    [6, 7],
    [6, 9],
    [1, 9],
    [1, 7],
    [0, 7],
    [0, 5],
    [2, 5],
    [2, 2],
    [0, 2],
]), "==", 62)


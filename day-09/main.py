class Location:
    x = 0
    y = 0

head = Location()
tail = Location()
tracker = {}
tracker['0,0'] = True

def move_tail():
    global head
    global tail
    global tracker

    h_dist = abs(head.x - tail.x)
    v_dist = abs(head.y - tail.y)

    # Still adjacent
    if h_dist < 2 and v_dist < 2:
        return

    # Non-diagonal move
    if h_dist == 0 or v_dist == 0:
        if head.x > tail.x:
            tail.x += 1
        if head.x < tail.x:
            tail.x -= 1
        if head.y > tail.y:
            tail.y += 1
        if head.y < tail.y:
            tail.y -= 1
        tracker[f'{tail.x},{tail.y}'] = True
        return

    # Diagonal move
    if head.x > tail.x:
        tail.x += 1
    else:
        tail.x -= 1

    if head.y > tail.y:
        tail.y += 1
    else:
        tail.y -= 1

    tracker[f'{tail.x},{tail.y}'] = True
    return

f = open('input.txt', 'r', encoding='utf-8')

for line in f:
    tokens = line.split()
    
    match tokens[0]:
        case 'U':
            steps = int(tokens[1])
            while steps > 0:
                head.y += 1
                move_tail()
                steps -= 1
        case 'D':
            steps = int(tokens[1])
            while steps > 0:
                head.y -= 1
                move_tail()
                steps -= 1
        case 'L':
            steps = int(tokens[1])
            while steps > 0:
                head.x -= 1
                move_tail()
                steps -= 1
        case 'R':
            steps = int(tokens[1])
            while steps > 0:
                head.x += 1
                move_tail()
                steps -= 1

print('Tracker size: ', len(tracker))


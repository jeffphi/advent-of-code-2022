class Location:
    x = 0
    y = 0

head = Location()
k2 = Location()
k3 = Location()
k4 = Location()
k5 = Location()
k6 = Location()
k7 = Location()
k8 = Location()
k9 = Location()
tail = Location()

tracker = {}
tracker['0,0'] = True

def move_tail(head, tail, record_move):

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
        if record_move:
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

    if record_move:
        tracker[f'{tail.x},{tail.y}'] = True
    return

    

f = open('input.txt', 'r', encoding='utf-8')

for line in f:
    tokens = line.split()
    direction = tokens[0]
    steps = int(tokens[1])

    # print(f'### {line} ###')

    while steps > 0:
        match direction:
            case 'U':
                head.y += 1
            case 'D':
                head.y -= 1
            case 'L':
                head.x -= 1
            case 'R':
                head.x += 1
        
        move_tail(head, k2, False) 
        move_tail(k2, k3, False) 
        move_tail(k3, k4, False) 
        move_tail(k4, k5, False) 
        move_tail(k5, k6, False) 
        move_tail(k6, k7, False) 
        move_tail(k7, k8, False) 
        move_tail(k8, k9, False) 
        move_tail(k9, tail, True) 
        steps -= 1

   # for k in tracker.keys():
   #     print(k)

print('Tracker size: ', len(tracker))


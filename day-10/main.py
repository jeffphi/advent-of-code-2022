
cycle_count = 0
register = 1
signal_strength = 0

def check_signal():
    global cycle_count
    global register
    global signal_strength

    char = '.'
    if abs((cycle_count-1)%40 - register) < 2:
        char = '#'

    #print(f'Cycle count: {cycle_count}, Register: {register}')
    match cycle_count:
        case 20 | 60 | 100 | 140 | 180 | 220:
            signal_strength += (cycle_count * register)
            #print(f'Interesting signal! Strength now {signal_strength}')
    if cycle_count % 40 == 0:
        print(char)
    else:
        print(char, end=' ')

f = open('input.txt', 'r', encoding='utf-8')

for line in f:
    tokens = line.split()

    cmd = tokens[0]
    match cmd:
        case 'noop':
            cycle_count += 1
            check_signal()
        
        case 'addx':
            cycle_count += 1
            check_signal()
            
            cycle_count += 1
            check_signal()
            
            register += int(tokens[1])

    #print(f'{line} cycle count: {cycle_count}, reg: {register}')

#print(f'Signal Strength Sum: {signal_strength}')


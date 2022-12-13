from collections import deque

class Monkey:

    def __init__ (self, items, op, op_val, test_val, target_t, target_f, test_val_old = False):
        self.items = deque(items)
        self.op = op
        self.op_val = op_val
        self.test_val = test_val
        self.target_t = target_t
        self.target_f = target_f
        self.test_val_old = test_val_old
        self.total_inspected = 0

    def inspect_items(self):
        global monkeys
        self.total_inspected += len(self.items)

        while len(self.items) > 0:
            worry = self.items.popleft()
            match self.op:
                case '*':
                    if self.test_val_old:
                        worry *= worry
                    else:
                        worry *= self.op_val
                case '+':
                    if self.test_val_old:
                        worry += worry
                    else:
                        worry += self.op_val
            
            # Adjust based on relief
            # print(f'{worry}/3.0 rounded down = ', end=' ')
            worry = int(worry/3.0)
            # print(worry)

            # Monkey Test
            if worry % self.test_val == 0:
                monkeys[self.target_t].items.append(worry)
            else:
                monkeys[self.target_f].items.append(worry)

monkeys = []

# Test Input
#monkeys.append(Monkey([79,98],'*',19,23,2,3))
#monkeys.append(Monkey([54, 65, 75, 74],'+',6,19,2,0))
#monkeys.append(Monkey([79, 60, 97],'*',0,13,1,3, True))
#monkeys.append(Monkey([74],'+',3,17,0,1))

# Real Input
monkeys.append(Monkey([83,62,93],'*',17,2,1,6))
monkeys.append(Monkey([90,55],'+',1,17,6,3))
monkeys.append(Monkey([91, 78, 80, 97, 79, 88],'+',3,19,7,5))
monkeys.append(Monkey([64, 80, 83, 89, 59],'+',5,3,7,2))
monkeys.append(Monkey([98, 92, 99, 51],'*',0,5,0,1,True))
monkeys.append(Monkey([68, 57, 95, 85, 98, 75, 98, 75],'+',2,13,4,0))
monkeys.append(Monkey([74],'+',4,7,3,2))
monkeys.append(Monkey([68, 64, 60, 68, 87, 80, 82],'*',19,11,4,5))

num_rounds = 20

for round in range(num_rounds):
    for i,m in enumerate(monkeys):
        m.inspect_items()
    
    print(f'After round {round+1}:')
    for i,m in enumerate(monkeys):
        print(f'Monkey {i}: {m.items}')

counter = []
for m in monkeys:
    counter.append(m.total_inspected)

print(counter)
counter.sort(reverse = True)
print(counter)
result = counter[0] * counter[1]
print(result)

# 66659 is too low

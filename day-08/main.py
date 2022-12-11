class Tree:
  height = -1
  seen = False

  def __init__ (self, height):
    self.height = height

grid = []
num_trees = 0

f = open('input.txt', 'r', encoding='utf-8')

for i,line in enumerate(f.readlines()):
    col = []
    for j in range(len(line)-1):
       col.append(Tree(int(line[j]))) 
    grid.append(col)

num_rows = len(grid)
num_cols = len(grid[0])
#print(f'Forest: rows:{num_rows}, cols:{num_cols}')

# Looking from the left, top to bottom
#print('From Left...')
tallest = -1
for row in range(num_rows):
    for col in range(num_cols):
        #print(f'[{row}][{col}] = {grid[row][col].height}')
        cur_tree = grid[row][col]
        if cur_tree.height > tallest:
            tallest = cur_tree.height
            if cur_tree.seen == False:
                #print('+')
                num_trees += 1
                cur_tree.seen = True
    tallest = -1

# Looking from the top, left to right
#print('From top...')
tallest = -1
for col in range(num_cols):
    for row in range(num_rows):
        #print(f'[{row}][{col}] = {grid[row][col].height}')
        cur_tree = grid[row][col]
        if cur_tree.height > tallest:
            tallest = cur_tree.height
            if cur_tree.seen == False:
                #print('+')
                num_trees += 1
                cur_tree.seen = True
    tallest = -1

# Looking from the right, top to bottom
#print('From right...')
tallest = -1
for row in range(num_rows):
    for col in range(num_cols - 1, -1, -1):
        #print(f'[{row}][{col}] = {grid[row][col].height}')
        cur_tree = grid[row][col]
        if cur_tree.height > tallest:
            tallest = cur_tree.height
            if cur_tree.seen == False:
                #print('+')
                num_trees += 1
                cur_tree.seen = True
    tallest = -1

# Looking from the bottom, left to right
#print('From bottom...')
tallest = -1
#count = 0
for col in range(num_cols):
    for row in range(num_rows - 1, -1, -1):
        #print(f'[{row}][{col}] = {grid[row][col].height}')
        cur_tree = grid[row][col]
        if cur_tree.height > tallest:
            tallest = cur_tree.height
            if cur_tree.seen == False:
                #print('+')
                num_trees += 1
                cur_tree.seen = True
    tallest = -1

print(f'Num trees: {num_trees}')

# Part 2

max_score = 0
for row in range(num_rows):
    for col in range(num_cols):
        up = 0
        down = 0
        left = 0
        right = 0
        cur_height = grid[row][col].height

        # Look up
        index = row - 1
        while index >= 0:
            if grid[index][col].height >=  cur_height:
                up += 1
                break
            up += 1
            index -= 1

        # Look right
        index = col +  1
        while index < num_cols:
            if grid[row][index].height >=  cur_height:
                right += 1
                break
            right += 1
            index += 1

        # Look down
        index = row + 1
        while index < num_rows:
            if grid[index][col].height >=  cur_height:
                down += 1
                break
            down += 1
            index += 1

        # Look left
        index = col - 1
        while index >= 0:
            if grid[row][index].height >=  cur_height:
                left += 1
                break
            left += 1
            index -= 1

        temp_score = up * down * left * right
        if temp_score > max_score:
            max_score = temp_score

print(f'Max_score: {max_score}')

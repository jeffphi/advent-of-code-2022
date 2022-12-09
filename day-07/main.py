class Dir:

    def __init__(self, name,):
        self.name = name
        self.sub_dirs = dict()
        self.files = dict()
    
    def __str__(self):
        return f'Name: {self.name}, dirs: {len(self.sub_dirs)}, files: {len(self.files)}'

    def get_total_size(self):
        total_size = 0
        # Add my file sizes
        for file_size in self.files.values():
            total_size += file_size
        # Add my sub_dir sizes
        for d in self.sub_dirs.values():
            total_size += d.get_total_size()

        #print(f'Total size of {self.name} is {total_size}')

        if total_size <= 100000 or self.name == '/':
            return total_size

        return 0

root = Dir('/')
cur_dir = root

f = open('input.txt', 'r', encoding='utf-8')

for line in f:
    tokens = line.split()

    # Is it a command? We care about "cd .." and "cd dir_name". We can skip "ls".
    if tokens[0] == '$':
        if tokens[1] == 'cd':
            if tokens[2] == '..': # cd'ing up to parent_dir
                cur_dir = cur_dir.parent_dir
            else: # cd'ing to a sub_dir
                cur_dir = cur_dir.sub_dirs[tokens[2]]
    # Or a dir entry
    elif tokens[0] == 'dir':
        temp_dir = Dir(tokens[1])
        temp_dir.parent_dir = cur_dir
        cur_dir.sub_dirs[tokens[1]] = temp_dir
    else: #it's a file entry
        cur_dir.files[tokens[1]] = int(tokens[0]) # nnnnnn filename 

total_size = 0
total_size += root.get_total_size()
print(f'Total size: {total_size}')
print(f'Root Info: {root}')




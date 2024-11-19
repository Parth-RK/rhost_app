
import os
from pathlib import Path
from collections import deque

# Function to check if the directory is accessible and writable
def is_accessible_and_writable(directory):
    return os.access(str(directory), os.R_OK) and os.access(str(directory), os.W_OK)

# Function to list directories with RW permission in a BFS manner
def pf(starting_dir, limit_dir):
    accessible_writable_dirs = []
    queue = deque([starting_dir.resolve()])  # BFS queue initialized with the starting directory

    visited = set()  # Set to keep track of visited directories

    # BFS loop
    while queue:
        current_dir = queue.popleft()  # Get the current directory from the queue
                
        # If we haven't visited this directory yet, check it
        if current_dir not in visited:
            visited.add(current_dir)

            # Check if the current directory is accessible and writable
            try:
                if is_accessible_and_writable(current_dir):
                    accessible_writable_dirs.append(current_dir)

                # Check all directories inside the current directory
                for entry in current_dir.iterdir():
                    if entry.is_dir() and entry.resolve() not in visited:
                        queue.append(entry.resolve())

            except (PermissionError, OSError) as e:
                # Log the error and skip the problematic directory
                print(f"Skipping {current_dir} due to error: {e}")
                continue

        # Stop if we've reached the limit directory
        if current_dir == limit_dir.parent.resolve():
            break

        # Move to the parent directory (BFS towards the root or limit)
        if current_dir != current_dir.root:
            parent_dir = current_dir.parent.resolve()
            if parent_dir not in visited:
                queue.append(parent_dir)
    # print("\nBefore Removal\n")
    # for directory in accessible_writable_dirs:
    #     print(directory)
    # Remove directories from CWD's parent upwards to the limit directory
    remove_parent_dirs_to_limit(accessible_writable_dirs, starting_dir, limit_dir)
    
    return accessible_writable_dirs

# Function to remove directories from CWD's parent up to the limit directory
def remove_parent_dirs_to_limit(accessible_dirs, starting_dir, limit_dir):
    current = starting_dir
    limit_resolved = limit_dir.resolve()
    limit_parent = limit_dir.parent.resolve()
    
    # Remove directories from the list that are in the path from CWD to limit_dir
    while current != limit_resolved and current != current.root:
        if current in accessible_dirs:
            accessible_dirs.remove(current)
        current = current.parent
    # Remove the limit directory and root directory explicitly
    if limit_resolved in accessible_dirs:
        accessible_dirs.remove(limit_resolved)
    if limit_parent in accessible_dirs:
        accessible_dirs.remove(limit_parent)
    root_dir = limit_resolved.root
    if Path(root_dir) in accessible_dirs:
        accessible_dirs.remove(Path(root_dir))

# Safety function (prevents disaster)
def safety(accessible_dirs):
    cwd = Path.cwd()
    current = cwd.name
    sandbox = 'subdir'
    if current != sandbox:
        accessible_dirs = []
        print("\nYou're not in a sandbox. Don't play with fire.\nOr Ignore it & just disable the safety.")
    return accessible_dirs

# Get the current working directory (dynamic starting point)
starting_directory = Path('.').resolve()

# Set the custom limit directory (you can adjust this to any desired limit)
limit_directory = Path(r'C:\Users\JOHN\Desktop\PRK')  # Change this to your custom path

# Get the list of accessible and writable directories using BFS
# accessible_writable_dirs = safety(pf(starting_directory, limit_directory))
accessible_writable_dirs = pf(starting_directory, limit_directory)
# Print the results
print("\nAfter Removal\n")
for directory in accessible_writable_dirs:
    print(directory)
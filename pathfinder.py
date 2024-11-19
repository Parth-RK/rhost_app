import os
from pathlib import Path
from collections import deque

# Check if the directory is accessible and writable
def is_accessible_and_writable(directory):
    return os.access(str(directory), os.R_OK) and os.access(str(directory), os.W_OK)

# List directories with read-write permission using BFS
def pf(starting_dir, limit_dir):
    accessible_writable_dirs = []
    queue = deque([starting_dir.resolve()])  # Initialize BFS queue with the starting directory
    visited = set()  # Keep track of visited directories

    # Perform BFS traversal
    while queue:
        current_dir = queue.popleft()

        # Check the directory if it's not visited
        if current_dir not in visited:
            visited.add(current_dir)

            # If the directory is accessible and writable, add to the result list
            if is_accessible_and_writable(current_dir):
                accessible_writable_dirs.append(current_dir)

            # Add subdirectories to the queue for further exploration
            for entry in current_dir.iterdir():
                if entry.is_dir() and entry.resolve() not in visited:
                    queue.append(entry.resolve())

        # Stop BFS when the limit directory is reached
        if current_dir == limit_dir.parent.resolve():
            break

        # Explore the parent directory to move towards the root or limit
        if current_dir != current_dir.root:
            parent_dir = current_dir.parent.resolve()
            if parent_dir not in visited:
                queue.append(parent_dir)

    # Remove directories from CWD's parent to the limit directory
    remove_parent_dirs_to_limit(accessible_writable_dirs, starting_dir, limit_dir)

    return accessible_writable_dirs

# Remove directories from the current working directory's parent up to the limit
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

    # Remove the root directory if included in the list
    root_dir = limit_resolved.root
    if Path(root_dir) in accessible_dirs:
        accessible_dirs.remove(Path(root_dir))

# Safety check to ensure the program runs only in a sandboxed environment
def safety(accessible_dirs):
    cwd = Path.cwd()
    current = cwd.name
    sandbox = 'subdir'

    # Disable actions if not in the sandbox directory
    if current != sandbox:
        accessible_dirs = []
        print("\nYou're not in a sandbox. Don't play with fire.\nOr Ignore it & just disable the safety.")
    return accessible_dirs

# Define starting directory and limit directory
starting_directory = Path('.').resolve()
limit_directory = Path(r'C:\Users\JOHN\Desktop')  # Adjust the path as necessary

# Get the list of accessible and writable directories
accessible_writable_dirs = pf(starting_directory, limit_directory)
# accessible_writable_dirs = safety(pf(starting_directory, limit_directory))


# Print the results
print("\nAccessible and Writable Directories After Removal:\n")
for directory in accessible_writable_dirs:
    print(directory)

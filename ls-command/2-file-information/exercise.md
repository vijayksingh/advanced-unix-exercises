
Stage 5: Retrieve and display file permissions

Your program should now display the file permissions for each entry in the directory. The permissions should be displayed in the format used by the `ls -l` command (e.g., "rwxr-xr-x").

Your program will be executed like this:

```
$ ./ls
-rw-r--r-- file1.txt
drwxr-xr-x directory1
-rwxr-xr-x executable_file
```

Unit test (test_stage5.sh):

```bash
#!/bin/bash

# Set up test directory
mkdir test_dir
cd test_dir
touch file1.txt
mkdir directory1
touch executable_file
chmod 755 executable_file

# Run the program
output=$(../ls | sort)

# Get expected output
expected=$(ls -l | tail -n +2 | awk '{print $1, $9}' | sort)

if [ "$output" = "$expected" ]; then
    echo "Stage 5 test passed"
    cd ..
    rm -rf test_dir
    exit 0
else
    echo "Stage 5 test failed"
    echo "Expected:"
    echo "$expected"
    echo "Got:"
    echo "$output"
    cd ..
    rm -rf test_dir
    exit 1
fi
```

Stage 6: Retrieve and display file owner and group

Your program should now display the owner and group for each file, in addition to the permissions.

Your program will be executed like this:

```
$ ./ls
-rw-r--r-- user group file1.txt
drwxr-xr-x user group directory1
-rwxr-xr-x user group executable_file
```

Unit test (test_stage6.sh):

```bash
#!/bin/bash

# Set up test directory
mkdir test_dir
cd test_dir
touch file1.txt
mkdir directory1
touch executable_file
chmod 755 executable_file

# Run the program
output=$(../ls | sort)

# Get expected output
expected=$(ls -l | tail -n +2 | awk '{print $1, $3, $4, $9}' | sort)

if [ "$output" = "$expected" ]; then
    echo "Stage 6 test passed"
    cd ..
    rm -rf test_dir
    exit 0
else
    echo "Stage 6 test failed"
    echo "Expected:"
    echo "$expected"
    echo "Got:"
    echo "$output"
    cd ..
    rm -rf test_dir
    exit 1
fi
```

Stage 7: Retrieve and display file size in bytes

Your program should now display the file size in bytes for each entry, in addition to the permissions, owner, and group.

Your program will be executed like this:

```
$ ./ls
-rw-r--r-- user group 1024 file1.txt
drwxr-xr-x user group 4096 directory1
-rwxr-xr-x user group 2048 executable_file
```

Unit test (test_stage7.sh):

```bash
#!/bin/bash

# Set up test directory
mkdir test_dir
cd test_dir
echo "This is a test file" > file1.txt
mkdir directory1
echo "#!/bin/bash\necho 'Hello, World!'" > executable_file
chmod 755 executable_file

# Run the program
output=$(../ls | sort)

# Get expected output
expected=$(ls -l | tail -n +2 | awk '{print $1, $3, $4, $5, $9}' | sort)

if [ "$output" = "$expected" ]; then
    echo "Stage 7 test passed"
    cd ..
    rm -rf test_dir
    exit 0
else
    echo "Stage 7 test failed"
    echo "Expected:"
    echo "$expected"
    echo "Got:"
    echo "$output"
    cd ..
    rm -rf test_dir
    exit 1
fi
```

Stage 8: Retrieve and display last modified date in a basic format

Your program should now display the last modified date for each entry, in addition to the permissions, owner, group, and size. Use a basic date format like "YYYY-MM-DD HH:MM".

Your program will be executed like this:

```
$ ./ls
-rw-r--r-- user group 1024 2023-06-25 14:30 file1.txt
drwxr-xr-x user group 4096 2023-06-25 14:31 directory1
-rwxr-xr-x user group 2048 2023-06-25 14:32 executable_file
```

Unit test (test_stage8.sh):

```bash
#!/bin/bash

# Set up test directory
mkdir test_dir
cd test_dir
echo "This is a test file" > file1.txt
mkdir directory1
echo "#!/bin/bash\necho 'Hello, World!'" > executable_file
chmod 755 executable_file

# Run the program
output=$(../ls | sort)

# Get expected output
expected=$(ls -l --time-style="+%Y-%m-%d %H:%M" | tail -n +2 | awk '{print $1, $3, $4, $5, $6, $7, $8}' | sort)

if [ "$output" = "$expected" ]; then
    echo "Stage 8 test passed"
    cd ..
    rm -rf test_dir
    exit 0
else
    echo "Stage 8 test failed"
    echo "Expected:"
    echo "$expected"
    echo "Got:"
    echo "$output"
    cd ..
    rm -rf test_dir
    exit 1
fi
```

These examples and unit tests cover the File Information feature, gradually adding more detailed file information to the output. Each stage builds upon the previous one, and the unit tests ensure that the new functionality works correctly while maintaining the previous functionality.

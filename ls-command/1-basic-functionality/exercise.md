
Stage 1: Set up project structure and implement a main function that prints a hardcoded list of filenames

Your program should print a hardcoded list of filenames and exit with code 0.

Your program will be executed like this:

```
$ ./ls
file1.txt
file2.txt
file3.txt
```

Unit test (test_stage1.sh):

```bash
#!/bin/bash

output=$(./ls)
expected="file1.txt
file2.txt
file3.txt"

if [ "$output" = "$expected" ]; then
    echo "Stage 1 test passed"
    exit 0
else
    echo "Stage 1 test failed"
    echo "Expected:"
    echo "$expected"
    echo "Got:"
    echo "$output"
    exit 1
fi
```

Stage 2: Implement reading and displaying contents of the current directory

Your program should now read and display the contents of the current directory, one file per line. Hidden files (starting with a dot) should not be displayed.

Your program will be executed like this:

```
$ touch file1.txt file2.txt
$ mkdir directory1
$ touch .hidden_file
$ ./ls
file1.txt
file2.txt
directory1
```

Unit test (test_stage2.sh):

```bash
#!/bin/bash

# Create a test directory with known contents
mkdir test_dir
cd test_dir
touch file1.txt file2.txt
mkdir directory1
touch .hidden_file

# Run the ls program
output=$(../ls | sort)
expected=$(ls -1 | sort)

if [ "$output" = "$expected" ]; then
    echo "Stage 2 test passed"
    cd ..
    rm -rf test_dir
    exit 0
else
    echo "Stage 2 test failed"
    echo "Expected:"
    echo "$expected"
    echo "Got:"
    echo "$output"
    cd ..
    rm -rf test_dir
    exit 1
fi
```

Stage 3: Add support for a command-line argument to specify the directory path

Your program should now accept an optional command-line argument specifying the directory to list. If no argument is provided, it should list the current directory.

Your program will be executed like this:

```
$ ./ls /tmp
file1.txt
directory1
file2.txt

$ ./ls
current_dir_file1.txt
current_dir_file2.txt
```

Unit test (test_stage3.sh):

```bash
#!/bin/bash

# Test with no arguments (current directory)
output1=$(./ls | sort)
expected1=$(ls -1 | sort)

# Create a test directory
mkdir test_dir
touch test_dir/file1.txt test_dir/file2.txt

# Test with argument
output2=$(./ls test_dir | sort)
expected2=$(ls -1 test_dir | sort)

if [ "$output1" = "$expected1" ] && [ "$output2" = "$expected2" ]; then
    echo "Stage 3 test passed"
    rm -rf test_dir
    exit 0
else
    echo "Stage 3 test failed"
    echo "Test 1 (no args) - Expected:"
    echo "$expected1"
    echo "Got:"
    echo "$output1"
    echo "Test 2 (with arg) - Expected:"
    echo "$expected2"
    echo "Got:"
    echo "$output2"
    rm -rf test_dir
    exit 1
fi
```

Stage 4: Implement basic error handling for non-existent or inaccessible directories

Your program should now handle errors when the specified directory doesn't exist or is inaccessible. It should print an error message to stderr and exit with a non-zero code.

Your program will be executed like this:

```
$ ./ls /non_existent_directory
Error: Cannot access '/non_existent_directory': No such file or directory

$ ./ls /root
Error: Cannot access '/root': Permission denied
```

Unit test (test_stage4.sh):

```bash
#!/bin/bash

# Test with non-existent directory
output1=$(./ls /non_existent_directory 2>&1)
expected1="Error: Cannot access '/non_existent_directory': No such file or directory"

# Test with inaccessible directory (assuming /root is not accessible)
output2=$(./ls /root 2>&1)
expected2="Error: Cannot access '/root': Permission denied"

# Test exit codes
./ls /non_existent_directory 2>/dev/null
exit_code1=$?

./ls /root 2>/dev/null
exit_code2=$?

if [ "$output1" = "$expected1" ] && [ "$output2" = "$expected2" ] && [ $exit_code1 -ne 0 ] && [ $exit_code2 -ne 0 ]; then
    echo "Stage 4 test passed"
    exit 0
else
    echo "Stage 4 test failed"
    echo "Test 1 - Expected:"
    echo "$expected1"
    echo "Got:"
    echo "$output1"
    echo "Test 2 - Expected:"
    echo "$expected2"
    echo "Got:"
    echo "$output2"
    echo "Exit codes: $exit_code1, $exit_code2 (both should be non-zero)"
    exit 1
fi
```

These examples show how the program should behave at each stage, along with unit tests to verify the behavior. The actual implementation is left to the developer.

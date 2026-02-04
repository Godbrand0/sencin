#!/bin/bash

# Test script for the student management system

# Create a test input file
cat > test_input.txt << EOF
add
John Doe
20
view
update
1
Inactive
view
exit
EOF

# Run the program with the test input
cargo run < test_input.txt

echo "Test completed"
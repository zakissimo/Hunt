#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <dictionary>"
    exit 1
fi

dict="$1"
files="./files"
bin="./bcrypt-source-code/bcrypt"

# Check if the file exists
[ ! -f "$dict" ] && echo "Dictionary not found: $dict" >&2 && exit 1
# Check if the directory exists
[ ! -d "$files" ] && echo "Directory not found: $files" >&2 && exit 1
# Check if the binary exists
[ ! -f "$bin" ] && echo "Binary not found: $bin" >&2 && exit 1

for bfe in "$files"/*.bfe; do
    while IFS= read -r pwd; do
        echo "Trying: ${pwd,,}"
        echo -n "${pwd,,}" | "$bin" "$bfe"
        [ $? -eq 0 ] && echo "<<<   We found a match: $pwd   >>>" && exit 0
        echo "Trying: ${pwd^^}"
        echo -n "${pwd^^}" | "$bin" "$bfe"
        [ $? -eq 0 ] && echo "<<<   We found a match: $pwd   >>>" && exit 0
    done <"$dict"
done

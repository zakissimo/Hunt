#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <dictionary>"
    exit 1
fi

dict="$1"
files="$(find "$(realpath ./files)" -type f -name '*.bfe')"
bin="./bcrypt-source-code/bcrypt"

# Check if the file exists
[ ! -f "$dict" ] && echo "Dictionary not found: $dict" >&2 && exit 1
# Check if the binary exists
[ ! -f "$bin" ] && echo "Binary not found: $bin" >&2 && exit 1

function brute() {

    bfe="$1"
    while IFS= read -r pwd; do
        echo -n "$pwd" | "$bin" "$bfe" >/dev/null 2>&1
        [ $? -eq 0 ] && echo "<<<   We found a match: $pwd   >>>    file: $bfe" && exit 0
        echo -n "${pwd,,}" | "$bin" "$bfe" >/dev/null 2>&1
        [ $? -eq 0 ] && echo "<<<   We found a match: ${pwd,,}   >>>    file: $bfe" && exit 0
        echo -n "${pwd^^}" | "$bin" "$bfe" >/dev/null 2>&1
        [ $? -eq 0 ] && echo "<<<   We found a match: ${pwd^^}   >>>    file: $bfe" && exit 0
    done <"$dict"
}

for bfe in $files; do
    brute "$bfe"
done

echo "No match found :'("

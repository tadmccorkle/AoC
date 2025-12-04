#!/bin/sh

if [ $# -ne 2 ]; then
    echo "Usage: $0 <year> <day>" >&2
    exit 1
fi

YEAR=$1
DAY=$2

EXIT_CODE=0
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
pushd "$SCRIPT_DIR" > /dev/null

if [ ! -d "./$YEAR/input" ]; then
    echo "Error: Invalid year provided." >&2
    EXIT_CODE=1
elif [ "$DAY" -lt 1 ] || [ "$DAY" -gt 25 ]; then
    echo "Error: Invalid day provided." >&2
    EXIT_CODE=1
elif [ ! -f "./session" ]; then
    echo "Error: No session ID file (./session)." >&2
    EXIT_CODE=1
else
    SESSION=$(cat ./session)
    DAY_PADDED=$(awk "BEGIN {printf \"%02d\", $DAY}")
    curl \
        --cookie "session=$SESSION" "https://adventofcode.com/$YEAR/day/$DAY/input" \
        > "./$YEAR/input/day$DAY_PADDED.txt"
fi

popd > /dev/null
exit $EXIT_CODE

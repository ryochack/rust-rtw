#!/usr/bin/env sh

CARGO_BUILD="cargo build --bin"
COMMAND="cat"
BUILD="debug"
RUST_TARGET_COMMAND="target/${BUILD}/${COMMAND}"

PWD="../"
CI_TEST_DIR="ci-tests/"
TEMP_DIR="${CI_TEST_DIR}tempdir/"

OUT_TEMPNAME="${TEMP_DIR}temp_out.txt"
EXPECT_TEMPNAME="${TEMP_DIR}temp_expect.txt"

if [ $# -lt 1 ]; then
    echo "require test data"
    exit 1
fi

TEST_DATA=$1

$CARGO_BUILD $COMMAND

for OPTION in -A -b -e -E -n -s -t -T -u -v; do
    echo "----- TEST: cat ${OPTION} -----"
    $COMMAND $OPTION $TEST_DATA > $EXPECT_TEMPNAME
    $RUST_TARGET_COMMAND $OPTION $TEST_DATA > $OUT_TEMPNAME
    diff $EXPECT_TEMPNAME $OUT_TEMPNAME
    if [ $? -eq 0 ]; then
        echo "PASSED"
    else
        echo "FAILED"
    fi
done


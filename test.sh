#!/bin/bash                                                                                      

test() {
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.s
  gcc -o tmp tmp.s
  ./tmp
  actual="$?"

  echo "Expected: $expected"
  echo "Output: $actual"

  rm tmp
  rm tmp.s
}

test 47 test.c

echo OK
#!/bin/bash

# Fetches the input, prints to stdout and copies to clipboard.
# This gives both a quick overview of what the input looks like
# and makes it available on ctrl+v for use in the challenge.

CONFIG="${CONFIG:-$HOME/.config/aoc.conf}"
YEAR="${YEAR:-2021}"

if [ -e $CONFIG ]; then
  . $CONFIG
fi

if [ -z "$1" ]; then
  echo "Please provide a day number."
  echo "usage: $0 DAY"
  exit 1
fi

if [ -z "$AOC_SESSION" ]; then
  echo "No session token."
  exit 1
fi

URL="https://adventofcode.com/$YEAR/day/$1/input"
curl $URL --cookie "session=$AOC_SESSION" -s | tee >(xclip -selection c)

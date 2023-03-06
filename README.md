# Aocio

Another Advent of Code I/O Helper in Rust

## Setting up your Session Key

`aocio` needs to know the Session Key you use for your Advent of Code account in
order to fetch input and send answers.

Here is how you find your Session Key:

1. Go to any [input page](https://adventofcode.com/2022/day/1/input) for Advent of Code
1. Inspect Element (right click then inspect or Ctrl-Shift-I)
1. Go to the Network Tab
1. Press Ctrl-R to reload the page
1. Click on the request with name "input"
1. On the Cookies tab inside that request, you should see the "session" cookie
1. Save the value of that cookie somewhere for future use

And now you are ready to use the `aocio` 🎉

## Usage

```bash
aocio fetch -y 2022 -d 1 -s "your-session-token-here" --overwrite-files # Saves Advent of Code input for Day 1 2022 to ./day1.txt and overwrites file if it already exists

aocio fetch -y 2015 -d 5 -s "your-session-token-here" -l "aoc/2015/" # Saves input for Day 5 2015 to ./aoc/2015/day5.txt

aocio fetch -y 2017 -s "your-session-token-here" -l "aoc/2017/" # Saves all 2017 inputs to ./aoc/2017/day{day}.txt

aocio submit -s "your-session-token-here" -y 2019 -d 7 -p 1 -a 2130 # Attempts to submit "2130" as a solution to 2019 Day 7 Part 1
```

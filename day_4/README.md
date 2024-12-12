## Do advent of code! it's awesome.
https://adventofcode.com/2024\
&emsp;\
&emsp;

## My findings:

Today's challenge was pretty easy all things considered. Rust has a few tricks that make this type of problem very easy with its iterators and enumeration features. I really like looping without the fear of an off by one error, especially when dereferencing a borrow.

My logic is a bit of a monstrosity. There was prohably a better way to do this, but making 5 or 8 relatively detailed if statements was just so easy, and allowed me to have an O(n) solution since it is just one iteration through each character. I will come back and update this if I think of a better solution to this problem.

I will be adding performance data to my reports from now on since it is relatively easy for me to include and measure how fast each solution is.

I have just been informed that we are supposed to not post our input. This is news to me but I am a bit of a newcomer to AOC. I will be scrubbing the input files from this solution and previous solutions.\
&emsp;\
&emsp;

## Performance data:

These results are the average of 3 runs on my computer with very little thought given to actual optimization.

Day 4, Part 1 averaged 1.61ms\
Day 4, Part 2 averaged 1.06ms\
&emsp;\
&emsp;


## Instructions follow:

--- Day 4: Ceres Search ---

"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS appear?

Your puzzle answer was REDACTED.

--- Part Two ---

The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

M.S
.A.
M.S

Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........

In this example, an X-MAS appears 9 times.

Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?

Your puzzle answer was REDACTED.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Shareon Bluesky Twitter Mastodon] this puzzle.


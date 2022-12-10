# aoc-2022
Advent of Code 2022

## Log

### Day 1

First day is usually easy and intented for setting up stuff. I used Belen's [aoc-2021](https://github.com/belen-albeza/aoc-2021) as a template for the project. I'm a bit *rusty* in Rust and no expert, so I just made a simple solution to get the stars in. When I get more experience maybe I'll find a better way for doing this.

### Day 2

Second day I started with a quick and dirty solution without any kind of Rust idiom. In the afternoon I had some free time that I used to clean the solution a bit and I used the `match` expressions quite extensively. Also I used a bit of time on figuring out how to debug from VSCode (using [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)). 

### Day 3

Nothing big today. For my first implementation I relied on `BTreeSet` to perform the intersection of the rucksacks. Then when cleaning the code I saw that the input wasn't that big and doing the vector intersection is faster and the code looks a bit cleaner. This has been a good exercise to play around with functional programming and understanding when to use `cloned` to avoid having to perform an extra `map` to dereference all the elements. 

### Day 4

The main difficulty as a Rust newbie has been parsing the input. I spent quite a lot of time battling with the `regex` crate and figuring out the proper way to get the list of range pairs. The problem itself was very similar to AABB collision, so after reading I more or less knew what approach I would take.

### Day 5

Today I had a busy morning so I settle for solving the problem with not the most elegant option. Since tomorrow is Independence Day here, I'll have some time to research a bit and clean up today's. 

After finishing day 6, I did a small cleanup of this solution starting by using the same `move_crates_from_stack` function for both implementations. That makes part 1 a bit slower, but simplifies the refactor. Then I took an approach where I implement the crate stacks as `String`, but after cleaning the input I realized that Rust `vec` has mostly the same operations than strings, so I kept is as arrays since it makes more sense. 

### Day 6 

Today the problem seemed pretty straightforward, but I ended up stumbling with a funny issue. Since I started using `u8` as my marker position, when running with the input it started failing, since Rust was wrapping around. I learned my lesson and will use `u64` from now on. 

### Day 7 

Things started getting interesting (it was a bit sad not having this problem yesterday when I had more time). I had a relatively clear idea of how to implement this from the beginning, but today it started with a bit of a fight against the *borrow checker*. I tested the `trees` crate but I ended up using `slab_tree` since it had a nicer API for what I needed. For the second part I wanted to have the function return an `Option` with the node and I ended up having to figure out the proper way of anotating the lifetimes, in the end I just return the size because the code looks cleaner but it has been a good learning experience.

### Day 8

For today I wanted a Grid so I used Belen's `utils.rs` class as a starting point. I didn't touch more the solution until I had some extra time to clean Days 8-10. I wanted to use the succint way of iterating on the grid I used last year with Python `product` so for this I used the macro `itertools::iproduct!`. 

### Day 9

Day 9 was the day I was feeling quite sick. I just implemented a really straightforward solution and didn't bother cleaning it up afterwards. I switched from char to `String` for stuff like the motion direction since `char32_t` show a little less nice in the debugger. I considered for a bit making the `follow_head` function get the tail as a mutable reference, but I switched to returning the new position as an `Option` since my code is advancing the knots step by step.

### Day 10

Looking at the example, I thought that maybe I should simulate the machine, but I realized that the approach suggested in the problem (adding up all the *addx* instructions) was probably much easier to implement. Then for part 2, since the 40x6 CRT is small enough I can just use my function to calculate the x values in each cycle and then use that to render the screen. When cleaning the solution I spotted a quite obvious way of speeding the code when the cycles that you request are in order, and that adds up in the CRT render. 

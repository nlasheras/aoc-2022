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

### Day 11

Today the parsing of the input was quite verbose but I got it relatively fast. For my first implementation I used the `eval` crate to avoid having to parse the *Operation* lines. I still had to figure out the trick to get the code able to run for 10k rounds. Even using the eval in each operation, the execution was around a second, but since it was quite impactful I refactored to have my own simple `Operation` enum that I can eval without having to parse each time. One of the small Rust challenges today was to access to two `Monkey`s at the same time. I had to fiddle a bit with my code to write something that the borrow checker would like. 

### Day 12

Today should have been much easier, but stumbled against the problem a long while until I realized that the `PriorityQueue` was not getting me the elements (it was returning them from higher value of priority to lower). For the second part a brute force A* is able to get the solution in under a second. 

### Day 13

Today was one of the days were half of the challenge is parsing and the other half is implementing the logic. The trick of using a recursive enum to implement it was possible in Rust, so that helped a lot. I opted for taking shortcuts in the solution, since I couldn't figure out the proper way of finding the sub slices (after getting the starts I took the time to figure it out). Also when doing part 2 I realized that my *compare* function was equivalent to `cmp::PartialOrd`, so I switched the solution to be a implementation it. The unit tests have been a great help during the parsing and initial implementation, since I could easily isolate the problem pairs and debug them step by step to find the problem in logic.

### Day 14

After a couple rough days, today I had a smoother experience with the exercise. Since I remember last year Origami puzzle, this time I assumed that using the grid won't be a good idea and started from having the rock walls in an array and then keeping a list of the "resting" sand (I ended up switching it to a `BTreeSet` for part 2, when there is much more sand to worry about). My initial implementation wasn't very clean, so the easiest way to solve part 2 was to duplicate the function and just hack the floor part there. I then later done the proper cleanup to have a helper `World` struct. 

### Day 15

Today it has been pretty tricky puzzle. The implementation was pretty straight-forward, but I needed a tip for part 2. I was maybe too stubborn trying to map other previous puzzles and didn't realize that if there is only one point, there is something that could be inferred from it. 

### Day 16 :warning:

Company christmas party today! Didn't have much time to work on this. My initial approach didn't work for input so I started to micro-optimize with no avail. After the party and a good night sleep I found that I had made a couple of wrong assumptions (AA is not always at the first element). Also the biggest optimization is that you can prune off the DFS around depth 8 in the real input because you won't have time to reach anything else. Still want to clean it and figure out why my input doesn't work in the input. 

### Day 17 

I was traveling today, so I couldn't invest much time on this. I squeezed some time here and there to start implementing it but finished it during the 18th. My initial implementation was a bit complicated to debug the issues since I chose the pivot on the center of the shape instead of the bottom right. I started maybe too long debugging bounding box issues while in the end just culling for distance was good enough. The trick of part 2 was pretty interesting to figure out, really enjoyed this one. 

### Day 18

This one was a very welcome easier one. When reading the puzzle I was expecting to have to figure out some smart spatial organization to query the cubes efficiently. But when looking at the input, with just a bit more than 2800 cubes it was enough to use a `BTreeSet`. 

### Day 19

My naive DFS simulation managed to get the answer for part 1, but proved unamanageable for part 2. I had to make it based on the "next robot to build" and still work out some culling of the solution space to get the answer in reasonable time (I'll leave it at 33secs until the end of the event).

### Day 20

For this one I had to fiddle a bit with part 2, until I realized that it's specified that the numbers have to be moved always in the order from the input. One cleanup I have pending is checking that the numbers are unique and I probably can avoid using the struct for them. 

# AdventOfCode 2020 Notes

These are the notes I made when solving Advent Of Code 2020 in Rust.

I didn't focus on describing solutions or complete thinking processes, so these are rather loose and slightly brittle notes, not a complete write-up.

The goal was to write about mistakes in thinking or quirks and problems with Rust I had in the process of learning it.

## Day (-1) - 2020-11-29

Installed Rust on Windows using [RustUp](https://rustup.rs/). I used the default options but then it turned out that I don't have all MSVC libraries so I switched to GNU toolchain with these commands from [rustup docs](https://rust-lang.github.io/rustup/installation/windows.html):

```shell
rustup set default-host x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu
```

## Day 0 - 2020-11-30

I made some preparations. First, I wanted to know how to include input files in the rust project. The program is compiled to a single binary file and I discovered there is an _easy_ way of embedding text files using `include_str!` macro.

These embedded files can be later assigned to a variable and read as a large string.

## Day 1 - 2020-12-01

These were simple problems but I had to remind myself a little about Rust. Types notation for simple functions were totally lost and I had to google how to parse `&str` to `i32`, too.

I stumbled on the [permutations](https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.permutations) function in the `itertools` create and some other strange iterators of pairs. But I'd prefer to stick to `std` as much as possible for now. I don't know  about producing iterative structures yet.

I tried to look for a function returning all pairs from `Vec`. Finally, I just used nested `for` loops. 

I solved it quite fast but spent some time refactoring and adding helper module `lib.rs` for parsing input files.

**Learned:**

- parsing strings: `[str].parse::<i32>().unwrap()`

## Day 2 - 2020-12-02

First idea was to parse using `split()` trait but I wanted to make it in a little more safe way. I was surprised that `regex` is a separate crate.

Searching for tokenization, I found these nice pages:
- https://nitschinger.at/Text-Analysis-in-Rust-Tokenization/
- https://adriann.github.io/rust_parser.html
- https://docs.rs/tokenizers/0.10.1/tokenizers/ - seems helpful for parsing

I thought about introducing a simple loop to parse these lines but finally I went for a simple `split`. I wanted to use `split_once` which would make things more readable but got shouted down by the compiler saying that this is new and not stable. How to use an unstable function? It was a mystery. After a while, I found that `#![feature(str_split_once)]` should do the thing but I didn't want to go this direction at this point.

Parsed lines are stored using `Line` and `Policy` structs. First part was easy as it required a simple `filter` and `count`. 

The second part was a little bit more interesting. I made a mistake by adding position to index in the password (instead of subtracting it) which led to `panic!`. So I used `match` for a two-value tuple. A little later I reminded myself about `if let` and used that instead.

I had small troubles with borrowing or iterators - it seems I don't get these well yet.

No problems with solving it conceptually but there was some Rust fighting. I found `lines()` which is now used in `lib.rs`.

**Learned:**

- can't easily use experimental functions like `split_once` - requires `#![feature(str_split_once)]`
- used `filter` and `count`
- found `lines()` function

## Day 3 - 2020-12-04

This day was a little easier since I understood more Rust basics. Creating a new `struct` wasn't a problem. I read about tuple structs which can be easily declared and I remembered about `enum`s too. Input parsing wasn't that hard. And the assumption I made was to create _methods_ over the `struct Map`. 

With these readable structures and the _method_ over `Map`, counting the trees wasn't that difficult. In other languages (F#), I'd try to write it in a functional way so the loop wouldn't be needed. But here, I had to use some `mut` variables. Fortunately, they were finally encapsulated into a function.

I still had to Google things a lot.

**Learned:**

- HashMap - used complex `struct Point` type for keys; it needs `#[derive(PartialEq, Eq, Hash)]` for keys
- use `for (key,value) in hashmap` to iterate over HashMap
- deriving `Debug` allows printing non-formatted data (like enums) with `println!("{:?}", enum)`
- `Iterator::product()` to multiply everything in a Vec
- there's no `do..while` in Rust - but it can be replaced by `loop { if <expr> { break; } }`
- `Iterator::enumerate()` iterates indexed values `(index, value)`
- `std::str::char_indices()` behaves similarly to `Iterator::enumerate()`
- there's a lot of referentials (`&`) in rust code (borrowing is still quite cryptic to me)

## Day 4 - 2020-12-04

Validation day! I felt that the fields' values would be necessary, so adding a dedicated `struct` was a good idea from the start. I tried to enable `#![feature(str_split_once)]` but it seems it isn't that simple. It might require upgrading to an unstable Rust version which I didn't want to do. Instead, I created a similar function locally in `lib.rs`.

The difference from F# is obvious in piping results of operations - it is not that simple in Rust. I wanted to pipe vectors of pair tuples to create a map. On the other hand, quite a lot of _piping_ can be done with chaining methods.

I didn't have many problems and wrote almost the whole part1 without Googling about Rust but validating and learning the library (functions for `String` and `str`) required it anyway.

At the end, after pushing the code to the repository it struck me... for validation I would normally use `Result` type. Instead of returning `bool` from functions, I should use Result. If I remember correctly, it has a really nice `?` operator for making the code linear. I could've changed it but let's leave it for the future–me so I can see my mistakes...

Writing in VSCode is really nice but when nesting deepens, support from the editor is getting worse - it is not quick enough as it probably needs to wait for types to be inferred properly.

**Learned:**

- the quickest way to debug is with `println!`
- re-learned: for converting between types use `as`, e.g.: `x as bool`
- adding static local fields - they should be written in capital letters and require the type to be specified, e.g. `static VAR: i32 = 2020;`
- trying to use `Option<&String>` in struct caused: `expected named lifetime parameter` 
    - trying to apply one caused `error[E0515]: cannot return value referencing local variable`
    - lifetimes are yet for me to capture...
- `String` has some nicer functions than `str`

## Day 5 - 2020-12-05

Mapping, looping and converting between types in Rust becomes more and more common to me. I still struggle with slicing but writing binary numbers was pretty natural: `0b0111`.

The problem was rather easy. My intuition wanted to operate on binary numbers straight ahead but I didn't discover that bits can be simply turned off. The first shot was to keep the max & min value of the number halves or to keep an array of numbers which would be cleared after each iteration. After seeing the bits on a calculator it became clear that "taking lower parts" is to disable a single bit.

**Learned:**

- binary numbers notation: `0b101`
- slicing (once again) - it surprised me that slices need to be made a **ref** types - `&my_str[0..7]`
- it is not possible to create a dynamic-size array, vectors should be used instead - `vec![INIT_VALUE; VEC_SIZE];`. Vectors are generally arrays in rust, they've O(N) access times and are continuous in the memory.

## Day 6 - 2020-12-06

The problems were very simple. With F# (or any language I know better) it would be very quick to solve it. 

The whole idea is based on finding unique characters in a specified text. In part1 it was easier because the search could be done on a whole group of answers. `HashSet` collection was useful to get rid of duplicate values. It could've been done by counting occurrences of each char in a 26-long array or by using the `dedup` function on a Vec. But I wanted to see how HashSets can be used.

It wasn't trivial to me to get my head around these mismatching types when using functional (chained) notation and I started with `for`. There is some cloning required sometimes or using references which are not always correctly suggested by the compiler. Part2 was easier to write in more functional way after finding help about using `fold` in the [issue 2023](https://github.com/rust-lang/rfcs/issues/2023).

Finally, I assembled some trivial tests and added a setup for a default test task in VSCode. The tests can now be run very quickly and nicely with `Shift+Alt+T`. I believe tests can replace debugging by `println!`ing in a more complex problems.

When I was searching for a flatmap, this popped out:
- https://www.eltonpinto.me/blog/posts/implementing_flatmap_in_rust/

**Learned:**

- using HashSet type
    - it has an interface similar to Vec
    - there is no macro like `vec!` for HashSets
    - it is not possible to easily intersect multiple sets - fortunately `fold` helps
- how to use `fold()`
- it is possible to specify the type when using collect function (similar to what comes with `parse`) like this: `.collect::<Vec<_>>()`
- re-learned: how to use tests (on a very simple samples)
- `Vec#dedup()` can be used to remove duplicate **consecutive** values

## Day 7 - 2020-12-08

Today's puzzle wasn't too hard either. Most of the problems this far concerned understanding how Rust works.

In order to understand the problem better, I drew a small graph for the first sample. It turned out to be DAG (directed acyclic graph). With the arrows added, it was very easy to understand where the real problem was.

In part1, the numbers weren't needed so I filtered them out to make debugging easier. I wrote a pretty long function checking whether the sub-collection is empty, contains "shiny gold" or else calling fn recursively. This was really readable, later it was refactored 

For part2, I had to modify the first parsing mechanism which didn't return numbers. This led to modifying part1 to ignore these numbers. The rest wasn't hard but I had to re-read the description which said about multiplying numbers, the thing I overlooked at the beginning.

Since there is no real REPL for Rust (AFAIK), I used tests to progress with the code. Tests have multiple advantages, like:

- they're very fast 
- can be run individually (in VSCode it's just a click on a test case)
- no requirement to create or modify original input file - sample input is just a `&'static str`
- (discussable if on plus) they're build separately and compiler might not complain about errors there

After a little struggle, I managed to refactor the code to nicer form without `if`s.

No problems with providing correct numbers, so far.

**Learned:**

- using tests instead of REPL
- `trim_end_matches` allows to provide custom text to trim
- calling fn recursively is hassle-free
- how to pass an iterator to function with `lines: impl Iterator<Item = &'a str>`
- `.to_string()` creates `String` values (just like `String::from()`)
- using `any()` on iterators works like contains but with `FnMut` as argument

## Day 8 - 2020-12-08

I like _Finite State Machines (FSM)_! Implementing them in functional languages is often tricky because of recurrence. Here, in Rust, I just used `loop`. It was nice and readable. I feel like avoiding mutables is probably a nice thing but overusing them is surely nothing nice.

Part1 was really cool. I'd like to have more ops to implement but I guess this FSM might be reused and extended in the following exercises.

Part2 surprised me a bit. I expected something a little bit harder. Although it was not, I didn't avoid refactoring the returned `i32` value to `enum` type. This was necessary to prevent code duplication: one function with two possible exit codes. Since there was just one _wrong_ op, it was very easy to check all cases where next operations were _fixed_. Fixing relies on providing a modified implementation of `get` function returning Op for provided index. The function is re-implemented for each pass. It would be better to avoid providing that many copies of local callback functions.

I wanted to hide private code unreachable to the topmost functions and found a way of doing so by introducing local modules (`mod`). It would only expose types and functions which are marked `pub`. As a result, these `enum` types do not _leak_ to the day8 module.

Tests really helped. I just used sample data and expected value and wrote the implementation until I got _green_. Compilation times and run times are really impressive. At this point I didn't observe any slowness in my implementations. Possibility of keeping the tests in the same file is really helpful.

Fortunately, I didn't fail when providing results in the first shot.

**Learned:**

- passing callback function to another with `where F: Fn(ARGS) -> RETURN_TYPE`
- using `loop`
- adding local `mod` to reduce visibility of types, fields and functions
- using `enum` types & adding `impl` for them
- using `panic!()` in default (`_`) match case – although I don't know about panicking almost anything
- once again(!) – it is not possible to declare dynamic-sized arrays, Vectors are used for that – `vec![TYPE; SIZE]`

## Day 9 - 2020-12-10

Today `for` loops ruled! The tasks were about finding numbers, longest ranges, summing and looping.

In part1, I wanted to use a function for finding all pairs but I couldn't find such a function in core rust libraries. I am still trying not to add external crates. There are no generator functions in standard rust, which I'd use to produce pairs manually. Or… they exist in the unstable version. I just had to write `for` inside a `for` to verify if there is a match. But apart from that, there is a really nice `windows` function which steps over all input numbers by 1 with a length of `preamble+1`. The last value in a window is the number and the rest is the preamble.

Part2 was not harder. I used `min()` and `max()` over slices first, which calculated the answer but I quickly refactored it to calculate min & max values on the go, without a need to reiterate.

As usual, tests helped by checking the sample code. The rules for the sample were a bit different (preamble length) so I had to test on a subroutine.

**Learned:**

- there are no built-in generators in Rust – they are worked on but are _extra unstable_
- re-learned slices – `x..y` (exclusive), `x..=y` (inclusive)
- slices can be declared without inner type specified, just like other generics: `slice: &[_]`

## Day 10 - 2020-12-11

Part1 was relatively easy to figure out how to sort the input.  There was nothing hard with counting differences of 1s and 3s. I provided the wrong result for the first time because I used the `u8` type when the result required a wider type. It was perfectly fine for parsing input but it was giving the wrong answer when multiplying two numbers(!). I just wrongly assumed that Rust would `panic` in case of exceeding a range of a type. 🤦‍♂️

Part2 on the other hand, took me a while to figure out. I knew I could use graphs to denote different "paths". I also correctly identified that it should be easy to count the possibilities from backwards. I even remember solving problems similar to this one. But I insisted on using a _counting_ variable. Writing it on a paper didn't help: I tried to count possibilities for every position in the array and then summing them or multiplying in some cases, otherwise just summing up… Drawing a tree helped a little but I had to think about walking up the tree and combining it with the written down array. This brought me to the _A-ha!_ moment.

One wrong guess because of number overflow. Other shoots were not missed.

**Learned:**

- there is no overflow checking for add or multiply - maybe it can be enforced?
- it is possible to `insert(POSITION, VALUE)` for a Vec
- to iterate over Vec in a reversed order: `(0..vec.len()).rev()` or better: `vec.iter().enumerate().rev()`
- [Ternary tree](https://en.wikipedia.org/wiki/Ternary_tree) exists!

## Day 11 - 2020-12-12

Both parts weren't very hard. But I spend quite some energy on loops. Again, tests help to continuously check if the code is still okay!

I'd like to generate DIRECTIONS with code but it seems like putting them into static is a good choice. Each loop has to use its own copy of the seat layout after each round. It is nice that whole layouts can be compared with each other. It might not be the most efficient but it is really helpful. And working!

**Learned:**

- `flatten()` is available for Iterable

## Day 12 - 2020-12-13

This is the first task I had to use _cartesian coordinate system_ a little bit more.

Part1 was pretty simple. The biggest trouble I had with rotating Directions by angle. It was nice to figure out that Directions can be numbered `0..3` and that it can be _moved_ by adding an angle simplified to `i32` (`angle / 90`).

In part2 all was quite understandable but I tried to rotate the waypoint _absolutely_ over a point where the ship is. Just for a brief moment, fortunately. I had an idea how a point should switch its signs for different quadrants but I wanted to come up with a little bit of a generalized idea. It was easier to set a rule of rotating by 90° in a loop by `angle / 90 mod 4`.

I used tests for all these rotating functions which was really helpful in catching problems. Maybe Point, Direction or Manhattan distance will be needed in the following days so I might want to reuse them.

**Learned:**

- how to declare operators – implemented `AddAssign` and `Mul` for Points
- implemented `From<>` traits for converting Directions to/from i32
- added tests to inner module
- implemented mutable structs with mutating functions `(&mut self, ...)`

## Day 13 - 2020-12-14

First fail (half-complete day)!

Part1 was smooth and easy.

For part2, I obviously wanted to run it brute-force but even for rust the calculation was very slow.
I saw that someone mentioned Chinese Remainder Theorem. I tried to grasp it but I had to give up.

**Learned:**

- `.ok()` converts Result to Option
- `max_by_key` exists on Iterator
- found `.div_euclid()` for primitive numbers (i32, i64, …). I didn't use it, though.

### Update 2021-04-09

Solved part2 after reading quite a lot about Chinese Remainder Theorem. It was quite tricky. The best lesson learned from this problem was to re-read the exercise again and ask the right questions.

## Day 14 - 2020-12-14

First part was quite easy and interesting to parse.
I failed on part2. I don't know where exactly the problem is.

**Learned:**

- parsing binary number with `u64::from_str_radix`
- binary operations
  - setting bitmask: `num | mask`
  - clearing bitmask: `num & !mask`

### Update 2021-04-07 00:40

Finally, after 10 trials, I managed to find the right answer. Falling back to the operation on strings helped. I must find what the problem was when  using numbers.

### Update 2021-04-07 01:31

I found it! In the bit clearing function there was an unwanted cast which caused the precision loss.

## Day 15 - 2020-12-16

That didn't seem hard. But I fixed myself on the vector searching…

In part1, it was easy to search through the `Vec`. I didn’t find a function that would return an index of the element on a vector (my stack). It could be easy to use the iterator for the `enumerate`. But that wasn’t quick enough. I wrote a function searching from the top of the stack.

Part2 required a different approach. I first tried with HashMap but it turned out it's easier with zero-filled Vec.

## Day 16 - 2020-12-17

Another validation day. Both exercises weren't very hard.

Part1 required simple validation of numbers. I started preparing structs for holding the data in some more meaningful (named) form. The validation itself was simple. At first, I put everything into `fn part1` but later decided to move it into dedicated methods of the structs.

I didn't have problems with part2. I wasn't sure the first answer would be correct – but it was.

I faced a problem with illegal `mut` borrowings in the moment when immutable ones were held. Another thing is that `for` loop _perfectly_ locks changes on the collection being looped. This led me to _iterating_ over indexes because numbers are copy-able and I didn't want to derive `Clone` or `Copy` in my structs.

For better performance, validations which were executed could be cached but since there were just a few hundred tickets that wasn't a big deal to run the checks multiple times.

I refactored a few lib.rs functions. And it seems I now know how to make some of the functions more generic.

**Learned:**

- how to make functions generic
- HashMaps cannot be compared - I ended up with comparing value by value

## Day 17 - 2020-12-18

Hard but not hard. I didn't even try to understand how these steps in the example were achieved. I tried to draw it on a paper, apply the strategy for creating new states but without success. Writing it in code seemed better (and easier). I just checked quickly how three-dimensional vectors could be created. But it struck me that I don't really need to iterate over dimensions. I just needed a map with locations and a boolean denoting a state. After confirming the parser is working, it took me just a little while to create the solution (a nested for-hell).

Part1 required 3D keys for the HashMap. Three-level `for` loops iterated over to find it. It requires to include cubes just behind the borders. Then for each of these locations there're another three-level for loops. They calculated the sum of active neighbours.

In part2, there is one more _dimension_ which I initialized with `0` from the original cube.

After small refactorings, it became clear that I don't need the `key` of the HashMap if there are only _active_ cubes on the list. I tried using `Vec` but that turned out to be veeery slow. Sets are better for this job. I tested `HashSet` but `BTreeSet` turned out to be a little faster (≈ 1.25s vs ≈ 0.95s).

**Learned:**

- `BTreeSet` (which was more performant than HashSet in this case)
- types can be aliased: `type Cube = BTreeSet<(i8, i8, i8)>` (which can help with readability)
- used `filter_map` for the first time

## Day 18 - 2020-12-19

I've done parsing by myself but running it the right order was not trivial. The post [Writing a Simple Parser in Rust, Adrian Neumann](https://adriann.github.io/rust_parser.html) ([GitHub project](https://github.com/adrianN/simple_rust_parser)), which I already mentioned before, helped greatly. I made a similar approach with tokenizing the input and then parsing it. Executing a parsed tree was another challenge but it turned out to be quite simple.

Not too quickly, but I eventually managed to finish part1. After small changes in the parser, part2 also passed.

That was a nice challenge because it required me to write a small parser. It was a nice reminder.

**Learned:**

- match binding with `@`
- making _recursive_ types requires value boxing: `Box<Token>`
- matching for the boxed values was also something new – please welcome the `&**value` (!)

## Day 19 - 2020-12-20

Nice task!

It took me a lot of time to find mistakes in my algo, thus I only finished part1.

### Update 2021-04-01

Part2 took me quite some time. I was continuing with the original idea to make it generic and it was very hard to put my head around it. After hours of failing and many days of shredded comebacks, I finally re-read the task and came up with the idea of preparing an expanded _tree_ of nodes. This made it easier to reason about recursion because it enabled continuations (picking next node).
Then, I guessed the maximum depth of recursion – starting from 10 which gave the same result as  4 (my fixed minimum). The first solution gave me the correct result but it took a couple of minutes to calculate.

I didn’t spend more time and it remains unoptimized.

**Learned:**

- `RUST_BACKTRACE=full` environment variable can be set to show panic details, although running the same code in tests produces more readable output
- vectors can be concatenated with `chain()` (https://stackoverflow.com/a/56490417/469961)
- this nice macro for debugging:
  ```rust
  macro_rules! dprint {
    // ($( $args:expr ),*) => { println!( $( $args ),* ); }
    ($( $args:expr ),*) => {};
  }
  ```

## Day 20 - 2020-12-21

I cannot spend much time on the problem today but I think I figured out how to approach it:

1. generate hashes for all borders of tiles
    - since borders are not too long (10 chars) it is possible to convert them to list of bools and further into `i32` using list of bools as bits of the number
    - `i32` numbers can be easier compared - no need to know which side it is placed
3. find tiles with only two matching hashes - these are our corners
4. find tiles with only three matches - these are the borders
5. for each corner try different "layouts" (rotations, flips) to detect which corner it suits best
    - in order to match corner to corner, there must be a continuous check until next corner is matched
    - no need to check "inside" tiles of the image

### Update 2020-12-27 23:30

Part1 turned out to be simple – it was just a matter of filtering out any tiles that have more than two matching borders. Only corner tiles have exactly two of them!

Part2 requires composing the proper image and it will not be that trivial.

### Update 2021-01-04

It took me some time but  I wanted to invest it to learn some new Rust stuff. I created my first `trait` and provided generic implementation of a few functions in it; implemented an iterator using a custom underlying struct. The implementation is not _perfect_ because it uses some cloning but I'm happy with it.

Could I use a hashset instead of Vec of Vec? I didn't want to try this but it would surely be doable.

**Learned:**

- implemented first `fmt::Display` for some structs (Tile, Monster and ActualImage)
- implemented my very first `trait` in Rust
- implemented iterator with custom underlying object
- used `.join()` function to combine strings

## Day 21 - 2021-01-06

This was as trivial as it seemed at the beginning. It required me to double–pass matching while searching the ingredients which match the allergens. First, I tried simple marking of all the ingredients (not repeated in other foods) but that failed hugely.

Part2 wasn't hard since I already had the pairs. I just had to change the structure from `Vec<Ingredient>` to `HashMap` and use it a little differently. HashMap has a very nice `contains_key` function. Then I just had to convert to a Vector of tuples, sort by allergen, get the correct ingredients and join.

**Learned:**

- HashMap can be _converted_ into vector of tuples with `iter().collect()`
- re-learned: single case pattern matching works nice in `map`
- standard library is quite poor: it would be nice to have functions like `all_pairs()`
- HashMap can be easily _created_ with `entry` function: `entry(???).or_default()`
- `filter_map` and `find_map` are really useful

## Day 22 - 2020-12-23

Completed just the first part. Didn't want to spend more time at the ATM.

### Update 2021-04-14

Second part was tricky. I spent a long time trying to figure out how to make it fully recursive. The solution I came up with worked pretty well for test data but it failed with stack overflow for the input. It took me a number of iterations to read and understand that it is just fine to recurse only for Games, leaving Rounds in the loop (like for part1).

**Learned:**

- re-learned: it's easy to `break` from a `loop` with a value - the break *can return* a value from the loop
- match expressions can be very long (wrapped with braces as any other expression)

## Day 23 - 2020-12-24

The first part doesn't seem to be hard but I can't do it right now.

### Update 2021-01-07

Solved part1. It wasn't hard. I used two `VecDeque` collections for subsequent moves. VecDeque also offers rotations – it was useful since I wanted to always keep the current element at the beginning.

### Update 2021-04-17

Solved part2 - it was tricky. Because the data in this problem has linked lists nature, it was hard to break connotation with the solution for part1.
I tried with a very long running, slightly optimized algorithm from part1 - but this obviously was never going to finish. Then I tried to find patterns, I thought the solution should repeat after some time. And indeed for part1 the pattern is becoming repeatable after 765 iterations. But finding the pattern for 1 million was impossible… or at least very hard.
Finally, I decided to use primitive but very quick Vectors. But it wasn't instantaneous: I tried with two vectors first (location → cup, cup → location). It turned out very quickly that this is not going to run because it was very costly to find "next" elements. All that just to figure out that simple single vector mapping, the current cup with a cup following it, is sufficient.

The same algorithm could obviously be used for part1 but I left the original solution.

**Learned:**:

- implementing iterators should be very simple for custom type
- used `Iterator#position(predicate: P)` to find index in a collection
- used `Vec<T>.windows(2)` to prepare the vector of relations

## Day 24 - 2021-01-08

Tried to implement it using graph structure. I had some problems implementing a recursive Node type but managed to figure out how to deal with `Option<Box<Node>>`. I ran the code and… got the wrong answer.

And then I searched and found this beautiful tutorial: https://www.redblobgames.com/grids/hexagons/

Implemented a simpler version and got blocked when giving an answer: 346. Which was surprising.

Tuned up the direction points a little bit and it worked.

### Update 2021-01-09

Part2 wasn't too hard but it introduced additional complexity. It could be resolved in a more optimal way with a loop which could make changes into the result set during its pass. Instead, I tried to make use of HashSet-dedicated functions.

**Learned:**

- implemented `Point<T>` type in my `lib.rs` – wrapped in a module and added basic operators
- it is possible to create type alias from generic to non-generic: `type Point = Point<i32>`

## Day 25 - 2021-04-17

This wasn't very hard to implement. The procedure was not perfectly clear at the beginning because I thought the _value_ was the same as the _subject number_. But reading it more closely, the solution was simple to find.

Part2 was given _for free_ (I just saw that for the first time on the AoC).

# Conclusions

Here's a bunch of conclusion after completing the problems:

- most often, there's no point to play with nice parsing - if the goal is to come up with a solution, wasting time on parsing is usually not worth it
- there is a quite narrow set of language constructs required for solving algorithmic problems like these - strings, vectors, sets, integers are the base others include looping, matching, transforming (map, filter) and some simple custom types
- `Vec<T>` is very fast (`O(n)`) and easy to use; there're even these nice `.push()` and `.pop()` functions which make it usable as a simple stack
- named or designed data structures (even very simple) help to abstract things when the complexity is high - use `struct` or `enum` or simple type alias, they're convenient!
- `BTreeSet` can be much faster than `HashSet`
- relying strictly on LF line endings can be tricky and sometimes give incorrect results while still not crashing
- to save some hassle when using some numbers as indexes, it's better to use `usize`
- when stuck, it help very much to break the problem and possibly **write tests for these parts**
- debugging
    - simply add `Debug` trait on types and use `[e]printfn("{:#?}", foo)`
    - other option is `dbg!()` macro

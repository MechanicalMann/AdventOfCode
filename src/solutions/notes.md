# Solution Notes

Some thoughts about my approaches to various days' puzzles and solutions, as a
retrospective. I won't be going in-depth here, just reminiscing.

Be warned, here be spoilers, for anyone who hasn't done the 2020 challenges and
who would like to.

## Day 1

I dove into this right after doing the first few tutorials from The Book, and
boy does it show. Most of my recent experience is languages with loosy-goosey
memory management and strong functional idioms (Typescript and C#), so you can
see why I gravitated toward the iterator methods initially. This definitely
comes back to bite me in the ass later on when I start battling with ownership.

## Day 2

I really didn't wait long until reaching for the Regex crate, huh? For someone
who hates Perl I'm awfully fond of Regex.

This was fun code to adapt from part 1 to part 2. I actually didn't look up
whether you could pass `fn`s by reference before I tried it - by this point I
had seen the type hints for closures in methods like `map()`, and I was able to
muddle through a generic type constraint using the compiler errors as a guide.
I have to say, the Rust compiler, despite being frustratingly strict, is usually
very helpful, or it tries to be. Definitely more so than other compilers!

## Day 3

Oof - I _really_ wanted iterator methods in Rust to be LINQ. They are not.
I accept that now.

I'm glad I worked out a solution that took virtually no adaptation to work for
part 2. I'm not so happy with the solution itself!

## Day 4

Oh Christ, I forgot about this one. Pure Regex abuse.

I can't remember why I didn't want to add the `lazy_static` crate for this one,
and I ended up adding it eventually anyway (to do more Regex stuff, naturally),
so who knows.

I also implemented an iterator for this mostly because I _thought_ Rust's
stable branch included a `yield` keyword. I was wrong, but then I kept going
anyway, still using the stable branch, for no particular reason.

There's just a lot of poor decisions in this one. This is gross, and I'm sorry.

## Day 5

This one was a lot of fun. I realized pretty quickly what this problem was, and
once you know that the solution is trivial. Of course, someone I know got fancy
with it anyway by (ab)using special properties of the bitwise XOR operator, but
I'm a simple man.

It's really interesting to me that a lot of Rust's number-parsing methods have
radix arguments, rather than making you implement the math on your own or hunt
down a different library/method. That came in really handy here (obviously) and
it seems like parsing things like binary or hex numbers in strings would be much
easier in Rust than in other languages.

## Day 6

Here, I finally discover the `HashMap` struct and begin anew my struggles with
ownership.

I know some people who struggled with this one, and with Day 4. I think not a
lot of people realize that line breaks are characters like everything else in a
string. Modern languages really do a lot to hide that kind of thing from the
developer I guess, for better or for worse.

## Day 7

This approach to "linked" lists makes a comeback much later in my solutions.
Using `HashMap`s to store and look up nodes rather than traversing a graph is
just better and nobody can convince me otherwise.

But what totally escaped me was to use another layer of abstraction to avoid
calling `clone()` on all these string references. Using numeric keys to bag
color names in another hash map would probably have reduced the ownership issues
I ran into here.

## Day 8

Despite having gone through tutorials in The Book, and keeping it open in a
browser tab literally all throughout this challenge, I forgot here that enums
can contain values, so I'm not very happy with the `Token`/`Instruction`
relationship here. I also should have gone all the way and turned `State` into
a virtual machine class containing the logic for interpreting instructions,
rather than having that on the `Instruction` struct itself. Very poor
separation of concerns there.

This was still a lot of fun though.

## Day 9

As you can see from the comment at the top of the file, I was still wondering
whether I would allow myself to go back and refactor previous solutions or if I
should leave them up as they were when I solved the puzzles. I ended up going
with the latter, but it came up here because I "borrowed" (and substantially
retooled) the `find_smarter` method from Day 1.

Anyway, it's not like I ended up making it capable of handling part 2, and I
ended up just reimplementing the Day 1 `find` (dumber) method instead of using
a hash set, which would've been cleverer.

## Day 10

No wonder I have several hundred thousand bags. Most of them must be full of
charge adapters.

This one annoyed me because I felt like there should be some way to do this
more mathily, but I just couldn't work it out.

## Day 11

The first [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
variant in the challenge, and it's pretty amusing. I liked the constraints,
though I'm not thrilled with my adjacency code (I tried a bit too hard to keep
it within the bounds of the map instead of just... discarding those points).

I ended up using a 2-dimensional vector here, because that's how I learned to
implement CGoL about a million years ago (C++ didn't have `unordered_map` back
then...), but in later CGoL challenges I switch to using `HashMap` to store a
sparse representation of cells indexed by coordinate.

The seeking code could use work too. I don't know why I didn't loop over the
8 directions, or why I used conditional patterns in the adjacency checks but not
in the seek code... oh wait, it was 1am on a Friday. Yeah, that checks out.

## Day 12

Whee, basically another virtual machine, but this time it's navigation. I'm
proud that this code was highly adaptable from part 1 to part 2 - all I needed
to do was add the layer of abstraction for the `Ship` that only intercepted the
`Forward` actions.

For things I'm _not_ proud of, feast your eyes on line
[85](https://github.com/malorisdead/AdventOfCode2020/blob/3b8846ca33fe43277b0e1448fa3ac86ae55de6dc/src%2Fsolutions%2Fday12.rs#L85).
This was the result of frantic trial and error, and can definitely be simplified
(the double modulo has to got go). But it works, which is all I cared about at
the time. I'm just glad the rotations were always increments of 90 degrees.

Note that I'm still not totally comfortable with the Rust ideas of abstraction.
My method placement is all over the place, sometimes associated with `struct`s,
sometimes bare functions. Honestly this continued throughout the challenge, and
I don't feel even now that I have a great grasp on what the idiomatic approach
is, or what I prefer personally. I have a very strong bias towards
object-oriented paradigms, but `struct`s are in a weird place where they're both
extremely limited groups-of-constant-size-values like C `struct`s, but they also
can have methods, implementations, traits, and other trappings of modern OOP.
This is probably one of those things that will come with experience and spending
time in the community, so I really just need to find more reasons to use Rust.

## Day 13

Ah, the first real "gotcha" challenge. I remember this one. Unfortunately,
this is the first challenge that I didn't really have fun with.

My solution to part 1 was fairly straightforward, if unoptimized - I should've
been keeping track of the minimum difference while looping over the buses, then
I wouldn't need to sort through them again after.

And then... **_Part 2_**.

![Fuck this](https://media1.tenor.com/images/e5cbae4ed0c3a2cc8fa218618c7a8144/tenor.gif?itemid=8936796)

The "gotcha" here is immediately apparent: the naïve solution will run for
approximately infinity (or at the very least many hours, possibly days). The
challenging portion is that it's not a simple least common multiple problem -
which was really annoying for me, because that idea lodged in my brain,
especially once I'd noticed that the puzzle inputs were all prime numbers. I
ended up spending an inordinate amount of time doing prime factorizations and
getting nowhere _before_ I noticed that, unfortunately.

I ended up using a spreadsheet and plugging a formula into Wolfram Alpha before
I finally noticed the pattern - that once you found a point of synchronicity,
the next one will occur at the next multiple of all the included numbers. So
instead of trying to multiply all the numbers together as I had been before, it
was a matter of keeping a rolling "multiplier" of the numbers whose multiples
had already been identified. I'm bad at math, so this is a poor explanation;
check the code, or find [someone else](https://github.com/k2mv/AdventOfCode2020/tree/main/13)
who can explain this better than I.

Ironically, as part of my desperate googling, I ended up on the Wikipedia page
for the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem),
which is essentially what this problem is. Unfortunately for me, that page was
clearly written by a mathematician _for other mathematicians_, and as of this
writing has not been edited down for general audiences - i.e., those of us
without number theory backgrounds - so the page was (and is) literally
incomprehensible to someone like me, and it ended up being no help at all.

## Day 14

And we're back to having fun! More binary goodness. I'm happy that I was able
to modify the code for part 2 without wrecking my part 1 solution. I'm less
happy with the unnecessary `Instruction` struct. By this point I'd implemented
methods for `enum`s before, so I don't know why I didn't do it here (I'm
assuming it's still my inbuilt bias toward thinking about these things in terms
of object-oriented languages I know, where `struct` == `class` and `enum` can't
have methods). I also should have used a `HashSet` to store the computed
addresses instead of just a vector, to reduce duplicate operations, but that's
a micro-optimization at best.

## Day 15

This one was less fun, but that was mostly my fault (I misread the documentation
for `HashMap.insert()` and spent a very long time wondering why it wasn't doing
what I thought it was supposed to). But once I scrapped the approach that
relied on nonexistent behavior, this was real easy. As you'll note, I didn't
need to change anything to get it to work for part 2, and it completes in about
3 seconds on my machine (which is admittedly pretty beefy). Perhaps the puzzle
assumes that people wouldn't use a hash map for part 1?

## Day 16

This one started out fun for me, then became less fun as I struggled with part 2
until I realized I had made a critical assumption about the data. My _theory_
was correct, and the approach itself works, but my validation code ultimately
assumed field values would never be zero, and ended up discarding one too many
tickets too quickly.

This is probably the first time that I felt the examples presented on the site
intentionally fostered that kind of assumption about the data, and obscured an
edge case that would ultimately make or break the solution. But it would not
be the last...

## Day 17

Our second Game of Life, this time **_in 3D!!!_**

![Put on your 3D glasses now!](https://jkirian92.files.wordpress.com/2014/10/wpid-3d_glasses_by_frantzkantor-d71vvna.gif)

Yeah, remember how I said I started using a sparse format to store my GoL cells
in later versions? Well I didn't do that. Oops. In fact, this code is
probably what pushed me to realize that my vector-of-vectors approach was really
not going to scale.

My solution is at once not that bad (it works! it's not slow!) but also pretty
bad (just so many nested loops!). At least it was relatively easy to update for
part 2, which is, naturally, **_in 4D!!!!_**

![Whoa](https://nega.bot.wtf/pix/images/riposte/recursive-shades.gif)

All it took was yet another level of nesting. Oh, and array growth. Oh, and
by making it 4D it doesn't work in just 3D anymore (damn hypercubes are adjacent
to things in the past and future... it's madness) so the solution can't work for
both parts at once, which is a shame. If I had been really clever I'd have done
something where the number of dimensions is variable. But I didn't, so here we
are.

## Day 18

Ok, despite being bad at math, I found this one pretty fun too. I think it's
just that I enjoy ASTs and expression parsing.

So here's where I confess: for this solution I pretty much just ported my code
from [Alea](https://github.com/malorisdead/Alea). Obviously, for the part 1
solution, I did make some changes (Alea respects traditional operator precedence,
for one), but I had basically written all this code before, just in C#.

That said, it was still neat to build, especially with custom precedence in
part 2. I was once again tempted to implement the tokenizer as an iterator,
but that seemed overkill to me this time. I think that would be a better
approach if I were building a more fleshed-out parser. This also got me
wondering how plugins for customizable language features would work - the way
traits are implemented it could be fairly easy to build an abstraction layer
around the expression builder.

Anyway, this was cool and fun and refreshing.

## Day 19

And here's where it all went wrong.

Part 1 was relatively easy, I thought, although I had a brain fart and forgot
that `Vec` itself has constant size (_members_ are heap-allocated) at compile
time, and that means they can be stored as enum values. So if you take a peek
through the git history you'll see my part 1 solution included separate enum
values for rules with one, two, or three references to other rules - oof.

But of course, that solution fell into the trap that part 2 set.

My initial solution, like probably everyone's, wasn't [greedy](https://en.wikipedia.org/wiki/Greedy_algorithm)
enough. It essentially returned a boolean (actually an `Option` with the number
of matched characters) as soon as it found a valid path, or if it found no valid
paths at all.

In hindsight, it's obvious why that wouldn't work for a validation algorithm,
but man did I spend a long time breaking my brain and printing out increasingly
complex representations of what was going on inside the validator.

Luckily, to make things better, a friend of mine came up with an example data
set that was shorter and easier to understand than the official example, and
which highlighted the problem with part 2. This was a huge help in debugging
my solution, and part of how I finally realized that I needed to evaluate _all
possible branches_ instead of just the first one that seemed valid.

I'm pretty happy with my solution in the end, which essentially collects the
number of valid characters for all possible branches, so if any of those numbers
match the total length of the string then the whole string must be valid. I'm
sure there are other nasty edge cases that would break that assumption somehow
but I spent way too long on this one anyway.

## Day 20

Back to having fun! I'm noticing a pattern with the days here, actually. I
wonder if that was intentional...

Anyway, here be dragons. This one was pretty straightforward, although I admit
I ended up having a lot of trouble with the rotation code when building a map.
[Line 299](https://github.com/malorisdead/AdventOfCode2020/blob/0e17648da1e13b6c2677859f8be08094e28a8a7f/src/solutions/day20.rs#L299)
in particular represents at least two hours of me bashing my head against the
wall. Of course, the next day I woke up, wrote out the permutations on paper,
and saw the pattern right away, resulting in the single-line solution that's
there now. Sometimes you just have to sleep on a problem to get it.

Once the map tiles were placed and rotated correctly, the rest was pretty easy.
I already had a lot of code to address individual "pixels" within the map from
printing data out as part of debugging, so I didn't even need to change much to
discard borders, show the whole map, and find monsters in it. So I spent a
little extra time making it look nice, too.

![Here be monsters...](https://share.bot.wtf/f/monsters.png)

## Day 21

Ok, maybe I'm wrong about the odd-even fun rule, because this one was pretty fun.
It's another "guessing" game where we need to iteratively eliminate items that
have been guessed from other possibilities. And once again, I fell prey to an
assumption about the data. Luckily, this one was much easier to work out -
needing to match _whole words_ because some "ingredients" can be substrings of
others, and doing a simple string match will turn up false positives.

Also, I again struggled a bit with mutability of collections and manipulating
them while looping. I'm content with my result here, which lets me simply
replace the collection with a new one from an inner loop, but I suspect there's
a better way to do it that I'm just not seeing.

Anyway, this was perhaps the first time for me that I solved part 1 in a way
that meant I had also solved part 2, I just needed to change what I was counting
for the answer. Nice!

## Day 22

![Time for crab](https://nega.bot.wtf/pix/images/time-for-crab.jpg)

Ok, this one should've been fun. Part 1 was! Part 2 was... less so.

I have another confession: I didn't really solve part 2. I have a solution, it
works, it got me the right answer, but it took possibly several hours. I don't
know, I left it running in the morning, and did actual work while it did its
thing. So I don't actually know how long it took, other than longer than one
hour, but less than five.

The [about page](https://adventofcode.com/2020/about) for the game states that

> every problem has a solution that completes in at most 15 seconds on
> ten-year-old hardware.

So clearly I have not found the correct solution to part 2.

I haven't spent much time thinking about what to do about it. I could revisit
it. I started down a path of memoization, I'm sure there are other things that
could be memoized to reduce repeat computations. Part of the problem with that
is the memory issue, though; the number of possible permutations of lists of
numbers that size gets unwieldy real fast. I had a thought about storing hashes
of states instead of the full arrays, but a glance at how Rust's hashes work
made that seem like more work than I was willing to put in to a silly puzzle
that was supposed to be fun. We're rapidly approaching the level of this being
as much work as my real life job at that point.

So here it is: my only wrong solution. At least I can take comfort in the fact
that it's _technically_ correct.

![And that's the best kind of correct](https://media1.tenor.com/images/7fb1df67990f0efbddf51c368dbe6919/tenor.gif?itemid=5787390)

## Day 23

🦀️🥤️

So Crabby wants a rematch, huh? Reminds me of the last time I was in Baltimore.

By my pattern, this one should've been annoyingly hard, but it wasn't actually.
For part 1 I was able to come up with a reasonably clever array-based solution
that worked by doing fun math with item indexes. It was a bit finicky to get
the math right but when it worked it was stupid fast and satisfying.

Then... _sigh_... part 2.

Ok, this was a bit annoying. I took a look at the parameters - ten million
iterations on one million numbers - and I knew the array index thing, clever
though it may be, would never work. I gave it the ol' college try anyway, and
sure enough, it didn't work. I realized I needed to throw the whole thing out
and start over, so I closed the laptop and went to bed.

The next day, in the morning, and during lunch, I tried several things.

First, I tried a couple different array-like things to see if I could swap
elements without visiting every single displaced element. I restructured my
loop, tightened up my math, tried deques and slices and all kinds of groovy
things, but no dice.

I realized I needed to use linked lists, so I could pop and swap items without
visiting literally one million other items. Rust has a `LinkedList` struct
built in! Yay! It doesn't support visiting arbitrary items in the stable
channel! Boo! So I upgraded my local install to use the nightly channel,
enabled the [Cursors](https://doc.rust-lang.org/beta/std/collections/linked_list/struct.Cursor.html)
feature, and tried that. I ran into two problems pretty quickly: cursors still
only let you "walk" down the linked list one element at a time, meaning there's
no way to move to an arbitrary element without visiting every element between
it and where you're starting, which was the whole reason I wanted to use linked
lists in the first place; and, on top of that, because this is Rust, I could
only have one mutable cursor at a time. Meaning that the final implementation
would have to walk to the place I was trying to swap elements into and then
walk back, two potentially huge operations per iteration.

Ok, fine. I'll implement my own damn linked list! ~~With blackjack! And you
know what this joke is pretty tired actually~~ I took a look around at some of
[the examples](https://rust-unofficial.github.io/too-many-lists/index.html)
(which are great) but it just made me realize that I wanted the best of both
worlds: arbitrary indexing _and_ constant-time swapping. Linked lists are only
fast to manipulate _when you have a pointer to the element you're manipulating._
I want it all, and I want it now!

So that's what I ended up with. My linked list is a hybrid, which uses a
`HashMap` to store nodes by value (this solution relies heavily on the fact that
we're storing unique, easily-comparable values), with each node containing a
reference to the _value_ of the next node. This very conveniently sidesteps the
major headache of implementing "actual" linked lists in Rust, which is that you
can't have infinitely recursive type definitions. All the by-the-book linked
list implementations have to jump through many, many hoops to ensure their
elements are heap allocated, but accessible, will be refcounted and disposed of,
etc. etc. This has none of that.

The upshot is that this implementation can also be circular, so I don't have to
have any extra logic to deal with wrapping around the tail of the list. It
ended up (I think) being even cleaner than my original array code, and it works
for both parts, with part 2 completing in just around 2 or 3 seconds on my
machine. The only downside is that I lose the ability to pretty-print the
current state of the cups for part 1, but considering how many elements there
are in part 2, that's not a feature I really miss much.

Naturally, this is the approach that nearly everyone I know took, but damn it
I've never taken a computer science class in my life so let me have my moment
of feeling moderately smart.

So ultimately this ended up being very fun, actually, because I enjoy this kind
of programming puzzle. It feels less like an arbitrary "gotcha" and more like
an actual problem that I had to think through to solve.

## Day 24

The second-to-last day! We're nearly at the end!

![Almost there...](https://nega.bot.wtf/pix/images/riposte/almost-there.gif)

And we're at our final Game of Life! This time, instead of being in multiple
dimensions, it's hexagons!

True to form, this even-numbered day was fun. Not only did I finally get around
to using a sparse representation, I ended up getting (I think) pretty clever
with coordinates. Thanks to a really wonderful explanation of hex grid systems
[over at Red Blob Games](https://www.redblobgames.com/grids/hexagons/), I was
able to use [axial coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-axial)
to represent my tiles, using only two integers for "row" and "column." This
did require me printing out some hex graph paper, filling in the coordinates,
and coloring the hexes by hand a bit to get the hang of it, but the end result
works really well.

One of my favorite bits about this was in overriding operators for my types.
Implementing `Not` for the color meant I could flip tiles easily with `!` and
implementing `Add<Direction>` for `Point` made navigating the instructions so
much easier.

My sparse representation of the cells is actually not _as_ sparse as I'd like -
I really should be _removing_ white tiles entirely to conserve space even more.
Luckily, that wasn't really a problem for this set of data, and would only start
to get worrisome after a few hundred or thousand generations.

Luckily, this was another problem where my approach for part 1 led naturally
into part 2. All it took really was adding a way to get the neighboring points
and the standard get/apply generational changes methods. Honestly, after this
month I feel like I've learned more about the Game of Life than almost anything
else.

## Day 25

Here it is, the big day! Finally, after all this waiting and anticipation!

What? _Christmas?_ Nah, fuck that - it's the last day of Advent of Code!

![AoC](https://media.tenor.com/images/8413357bafa6b8bf2594b954c3934c65/tenor.gif)

One final confession: after day 19 I was so behind that I didn't even start this
until the 26th. But despite it all, I soldiered on!

For the final act, a single one-parter that reinforces the story that this elf
is a criminal hacker who needs to be stopped. Cracking public key exchanges!
What's next!

Anyway, probably as a Christmas present to everyone who participated, this one
was, despite its nefarious goals, really easy. I guess the only trick is
paying attention to the wording so you don't mix up the second stage.

And that's it! We're done!

## Final Thoughts

Overall, I think I did have fun. Toward the end, it felt like I wsa spending a
lot of time on solutions, which wasn't the case earlier in the month, despite
the language being new to me. Ultimately I think this helped my cause, though.
The more I learned and grew comfortable with Rust, the more difficult the
problems were that I had to solve with it.

As I've noted, it did sometimes feel like there were "trick" questions, or
"gotchas" where the test data implied some assumptions about the actual puzzle
inputs that were not true. I think this is the less entertaining part of the
puzzle, because so many people use the example data to verify their solutions
before running them on the actual inputs. I do wish the examples wouldn't do
that as much, and I hope next year will have fewer of these cases.

For me, the most fun puzzles were ones that required complex thinking around
the approach. The interesting variations of the game of life, expression trees,
linked lists: those were all lots of fun for me, and I think for others. The
only "gotcha" was that the first-pass solution wouldn't work, and a change in
thinking would be needed to reach the second. Usually those also involved a
natural ramp-up between parts 1 and 2, with part 1 employing the simple solution
and part 2 requiring the new approach. That felt natural and clever to me.

Anyway, as for Rust: I'm actually coming around to liking this language. I
remember learning C and C++ a long time ago, but being frustrated with the
manual memory management overhead. Rust seems to me to offer the low-level
performance of those languages but with enough safeguards around memory to keep
me from null-pointering myself into oblivion.

It's definitely a challenging language to work with. Those safeguards really
force the developer to think and think hard about what their data is doing and
why they would need to mutate it, or even if they should at all. This is good.
Despite the slight overhead, I found myself appreciating the times when I had
to dereference a variable, or explicitly reference something. It's so much
less work than allocating and deallocating blocks of memory, but it's a good
reminder about what's going on. Do I really _want_ to copy that value here?
Would a reference be better? Does this reference need to be mutable?

So while I would call Rust a challenging language, I think it's challenging in
a good way. Writing in Typescript or C#, I don't often find myself thinking
about how memory is being allocated and how accessing that memory might cause
problems until those problems happen, usually at runtime (and sometimes in
production). It's easy to rely on garbage collection until something goes wrong.
In Rust, though, every time I pass a value to a method or function, I have to
think about what's really happening. If I try to pass a struct by value, and
I haven't implemented `Clone` and `Copy`, the compiler throws an error at me.
That makes me think - do I want to be able to pass these by value, or should it
be a reference? What am I actually trying to do? It's usually not a hard
question to answer ("oh, it's just two integers, I can pass that by value" or
"oh, this actually has a pretty deep nested structure, I don't want to clone
that much") but I'm glad that the language asks me to answer it anyway.

So yeah. Despite some rough times, I had a lot of fun doing this, and I'm
looking forward to doing it again next year. Will I use Rust again? Learn
another language? Who knows!

In any case, if you've read this far, thanks! Happy holidays, and I'll see you
in 2021!

🎄️ 🎅️ 🌟️ ❄️ ☃️

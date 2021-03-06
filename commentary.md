# A Commentary on Life

Let's start by establishing what exactly it is that I'm setting out to
implement. The Game of Life is a two-dimensional cellular automaton that was
created by John Conway in 1970. It consists of a two-dimensional matrix of
'cells'. Each cell can be in one of two states: 'alive' or 'dead'. There exists
a 'clock' whose 'ticks' shall be called 'cycles'; once each cycle, the state of
every cell is updated according to a set of rules, which are dependent on the
states of neighboring cells. The rules are as follows:

1.  Any living cell with either two or three live neighbors remains alive.
2.  All other living cells die.
3.  Any dead cell with exactly three live neighbors becomes alive.
4.  All other dead cells remain dead.

Typically, the matrix of cells is said to be infinite. However, with finite
computational resources, I'll need to use a finite subset of the infinite
plane. There are tricks and techniques that I could use to simulate the full
infinite plane, but they require more thought and research than I am intending
to invest in this project.

Among finite implementations of the Game of Life, there are two variants: those
with wrapping boundaries (effectively a torus), and those with absolute
boundaries (for the purposes of computation, these have a buffer ring of cells
around the matrix which are in a constant dead state). For no reason in
particular, I'll be implementing the wrapping sort.

The first thing that jumps out at me is that, as it is necessary to calculate
the next state of each cell before any cells are updated, it will be easiest to
maintain two separate copies of the matrix. Only one copy at a time will be the
'current' copy, the other being used to store the computed values of each cell
until the entire matrix is updated. To 'update' the matrix, we simply have the
two copies switch roles - after all the computation is complete, of course.

With that in mind, I think that a good place to start will be to implement some
data structure designed for storing the state of the matrix and performing
relevant operations on it. And so the aventure begins.

> # Commit:
> ### preamble
> 4b86426c78b4b300f56581a084a66ce0c5807e39

As the matrix data structure represents the 'world' that the cells inhabit, I'm
going to name it `World` (and place it within a corresponding `world` module).

The `World` structure I've come up with has the following fields:
-   `current: bool`: which copy of the matrix is the current
-   `w: usize`: the width of the world
-   `h: usize`: the height of the world
-   `a: Vec<Vec<bool>>`: one of two copies of the world
-   `b: Vec<Vec<bool>>`: the other of two copies of the world

It's also got a `World::new` associated function that initializes a new world
with a given width and height, filled with dead cells.

> # Commit:
> ### create world structure
> 0c857717d777ab6f15b8567c4e9f797b2e84a4bf

The next thing to think about is operations that will be performed on it. There
are two general classes of operations.

The first class is operations performed by external code. These operations will
exclusively interact with the current copy of the world, and fall into one of
two subcategories:
1.  Reading from the current copy of the world (for example, to render it)
2.  Writing to the current copy of the world (for example, to load the starting
    state of the world)

The second class is operations performed by internal code. These operations
almost exclusively interact with the intermediate copy of the world, as the
vast majority of them will be performed by the methods that compute the next
world-state.

For these reasons, it is necessary to have a method that allows indexing into
the current copy of the world, and a similar one for the intermediate copy.

At this point, it occurs to me that as I'll be implementing a wrapping version
of the Game of Life, it would be helpful if these indexing methods allowed for
signed indices which will automatically be mapped to a point within the world
through something resembling modulo arithmetic. Of course, if the world is to
be indexed using `isize`s, it should never be permitted to initialize a world
to dimensions larger than `isize::MAX`. This restriction will have to be added
to the `World::new` function.

> # Commit:
> ### restrict dimensions in constructor
> 3ed23d35030cdbecddc8048e02aa532943fec17c

The indexing functions are fairly easy to implement. They take an `isize` for
the width and an `isize` for the height. These are normalized to fit within the
bounds of the world, which for non-negative indices is done with a simple
modulus, and for negative indices it is just a matter of repeatedly adding the
appropriate dimension until it is non-negative. The indexing functions return
a mutable reference to the relevant cell.

> # Commit:
> ### add `.c(_, _), .i(_, _)`
> bfd39324c821112ad9eef8ffd6b4fb1dca4b972b

This is the point at which I remember to do something that I'd forgotten: check
to make sure that my code compiles. Alas, it does not. I fix up the errors and
make another commit. Embarrasingly, I discover that when I copy-pasted the code
for the `.c(_, _)` method into the `.i(_, _)` method, I forgot to change the
logic such that it indexes into the intermediate matrix instead of the current
one. Oops.

> # Commit:
> ### correct compilation errors
> 4c1f381dac1f2d5dd5fc4a30b1fc21ff24b64d11

At this point, I want to implement a debug formatter so that I can print the
state of both matrices and check that everything is working correctly.

It should be a simple matter of iterating over the matrices and printing each
cell. Dead cells will be associated with a space, while living cells will use
the "filled box" character (???).

Implementing the `fmt::Debug` trait reveals a miscalculation I made: in an
attempt to reduce complexity, I created only two indexing functions, both of
which return mutable references. This is a problem because it requires a
mutable reference to `self` to begin with. I'll clone both functions so that
there can be an immutable and a mutable version of each. Since the immutable
versions... don't need to be mutated, and since the value of each cell is only
a single boolean anyway, the immutable versions will just return the value of
the cell without being wrapped in a reference. In all honesty, I'm so annoyed
with this system that at some point I will probably return to this and figure
out a way to implement `ops::Index` and `ops::IndexMut` on `World`, but that's
not my current focus.

> # Commit:
> ### implement `fmt::Debug` for `World`
> e9599fe290a1209ae2e587ffefd5cf11a42a7cfa

Now that I've implemented `fmt::Debug`, I think it's finally time to write some
tests. I'll just put these in `main` for now. Since the intermediate copy is
only accessible to code within the `world` module, I'll only be writing tests
for the current copy for now.

The test I wrote looks something like this:

```rs
let mut world = World::new(8, 6);

*world.c_mut(0, 0) = LIVE;
*world.c_mut(-1, -1) = LIVE;

println!("{world:?}");
```

The expected output is that it will print out the current copy, which should
have a live cell in the top left corner and in the bottom right corner, as well
as the intermediate copy, which should be completely blank.

The first time I ran the test, it did indeed print a blank grid for the
intermediate matrix, and the current grid had the correct corners highlighted.
However, since I had made the world rectangular instead of square, I was able
to notice that somewhere along the line I'd flipped the horizontal and vertical
axes. Skimming through my code, I quickly noticed that the issue was that I'd
been iterating first through the columns and then through the rows in a nested
loop - however, since printing occurs on a row-by-row basis, I should have been
iterating first through the rows, and then through the columns in a nested
loop. After fixing that, re-running the test demonstrated that all was in
order.

> # Commit:
> ### correct `fmt::Debug` implementation
> d2f29dcc9e1ae612e5875ebd785b9bf86e9054aa

> In all honesty, I'm so annoyed with this system that at some point I will
> probably return to this and figure out a way to implement `ops::Index` and
> `ops::IndexMut` on `World`, but that's not my current focus.

The time has come.

Basically, what I want to be able to do is this:

```rs
let mut world = World::new();

world.c[0][0] = LIVE;
world.c[-1][-1] = LIVE;
```

That's the ideal anyway. I may have to settle for indexing by a tuple of the
x and y values. We'll see.

---

I've spent some time thinking about it, and I've come to a realization: any
external code will only ever need to - or, in fact, be able to - index into
the current copy of the matrix. Therefore from an outsider's perspective, being
forced to index into the matrix through a field is completely redundant. Now,
originally, my justification for this system was that if I implemented the
indexing operators directly on `World`, that would leave no indexing API
available for the intermediate copy of the world. What I've come to realize
though, is that that's really not a big deal at all. What I've done in my
`fmt::Debug` implementation, I can simply repeat for all the other methods in
the `World` impl. That is, each method can simply begin with a match statement
that retrieves a mutable reference to the correct vector, and the rest of the
function will simply use that mutable reference. This significantly simplifies
my task, since what I would have done otherwise (with two additional fields)
would have required me to create two new types, or if I got irritated enough
to accept defeat and index by a tuple, one. Not only that, the custom type of
the fields would have needed to somehow store a mutable reference to the parent
`World` object, and while I've admittedly not even attempted it, I'm not sure
that Rust's ownership semantics allow that. (I suppose I should mention that as
the author of the `bcbypass` crate I am really not that averse to using healthy
servings of unsafe code, but that's besides the point.)

---

After implementing the `ops::Index` and `ops::IndexMut` traits, I had to
rewrite my implementation of the `fmt::Debug` trait as well, to use the new
indexing system. For the current copy, this was a joy:

```rs
...
match self[(x, y)] {
    ...
}
...
```

However, when I began to rewrite the part of the `fmt::Debug` implementation
that prints the intermediate matrix, I quickly realized that without some sort
of indexing function I would not only need to rewrite the logic for determining
which matrix was the intermediate one in every method, but also the logic for
normalizing the indices to within the bounds of the world. I decided this was
going to be too much of a hassle, and so I kept the `i` and `i_mut` methods.
That said, I did refactor them slightly (by which I mean, I copy/pasted the
code from the `ops::{Index, IndexMut}` impls) and I renamed them to `inter` and
`inter_mut` so that it is clearer what their functions are.

> # Commit:
> ### refactor indexing api
> 2eac16f977760da7a4d713f59e36acd96c0d58d4

This one will be short. At the suggestion of a friend, I'm replacing the `bool`
field (`.current`) as well as the `bool`s used in the matrices with proper
enums. I don't know why this didn't occur to me in the first place.

> # Commit:
> ### replace bools with enums
> 4d375709b6aad3a9c018cc24e2278e9269c64886

At long last, the time has come. I've run out of excuses to procrastinate. It's
time to implement the rules of the Game of Life.

I'll be implementing them via a method called `cycle`. Every time the method is
called, it'll iterate through the current copy, writing the new state to the
intermediate copy, and then when that is finished it will swap the two.

---

It is done. Surprisingly, it was less painful than I'd anticipated. We'll see
if it actually works!

> # Commit:
> ### implement cycle method
> b7059eff36fa68cb2327c4ee36a107c39b971e76

Ugh. It didn't work. More compile errors.

> # Commit:
> ### correct compilation errors
> ed94e005ceb37ac980d5d2b85e6bd342e51b4dfb

Great! Now we can write a real test and see if I've implemented everything
correctly. My first test will be a simple 2-cycle. According to the rules of
Life, three live cells in a line surrounded by dead cells should alternate
between vertical and horizontal forever. Here's the test:

```rs
let mut world = World::new(5, 5);

world[(2, 1)] = Cell::Live;
world[(2, 2)] = Cell::Live;
world[(2, 3)] = Cell::Live;

println!("{world:?}");

world.cycle();

println!("{world:?}");

world.cycle();

println!("{world:?}");
```

> # Commit:
> ### test with 2-cycle
> dda28732dd9581c414e8f005947e14423e973d6f

Alas, all is not well. For whatever reason, the intermediate copy is never
changed - so when a cycle happens, the only thing that happens is that the two
copies switch roles. I suspect that the cause of this is some error in my
`inter` method resulting in none of the writes ever actually occurring.

---

...

Man I feel stupid. Nothing was wrong with the `inter` method at all. Luckily,
I remembered that I had literally copy/pasted the method's code from my
`ops::IndexMut` implementation, which I already knew worked. That prevented me
from following this red herring too far down the rabbithole. No, the problem
was in the code of the `cycle` method. Closer inspection of it revealed a
subtle mistake: instead of using `x` and `y`, which were the loop variables,
I'd used `w` and `h`, which refer to the width and height of the world and in
this context are effectively constants. Note to self: using an entire word as a
variable name is ok - good, even. (Of course, just because I wrote that note
doesn't mean I actually changed the variable names. I am a masochist after
all.)

> # Commit:
> ### correct cycle implementation
> 81b5a19c056f8784efcd599d22379314859c511d

Well, now that that's fixed, I ran the test again and it worked perfectly! It's
a bit hard to tell because the non-square nature of terminal characters fucks
up the dimensions of the line, but it is a three by one line that alternates
between vertical and horizontal!

For a more comprehensive test, I'm going to make a glider. Here's the code:

```rs
let mut world = World::new(10, 10);

world[(2, 1)] = Cell::Live;
world[(3, 2)] = Cell::Live;
world[(1, 3)] = Cell::Live;
world[(2, 3)] = Cell::Live;
world[(3, 3)] = Cell::Live;

for _ in 0..20 { world.cycle(); }

println!("{world:?}");
```

A glider moves across the screen in a diagonal direction. The way that I've
written it, the glider will move towards the bottom right. The period of the
glider's cycle is 4 world cycles, and after each cycle it will have moved 1
unit in its direction of travel. Therefore, after 20 cycles, we should expect
the glider to have moved down 5 cells and to the right 5 cells. With world size
set to 10x10 and the glider beginning in the top left, we should expect to see
it end up in the bottom right. Let's test it out!

---

It worked perfectly! Now I could be wrong but at this point, I feel comfortable
claiming that I've implemented Conway's Game of Life. Success!

The next stage of this project will be to figure out how to properly render
the world in a window.

> # Commit:
> ### test glider
> 19598313a08030ad42e1ef954e713f1fa57d76ba


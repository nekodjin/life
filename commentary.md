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
> ### add .c(\_, \_), .i(\_, \_)
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
the "filled box" character (█).

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


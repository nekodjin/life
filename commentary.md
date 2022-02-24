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


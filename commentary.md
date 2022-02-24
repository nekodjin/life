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


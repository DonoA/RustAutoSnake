RustAutoSnake
===

Rust auto snake simulates a perfect game of snake using various pathing AIs. The project uses ncurses to enable simple commandline rendering.

## Usage

Rust auto snake depends on ncurses:
```sudo apt-get install libncurses5-dev libncursesw5-dev```

Once ncurses is installed, the project can be built and run using cargo
```cargo run```

The user should be presented with a screen like the following:
![](https://raw.githubusercontent.com/DonoA/RustAutoSnake/master/simple_screen.png)

## Controls
* `F1` Exit
* `Space` Pause/Unpause
* `w` increase time between ticks, slowing down the game speed
* `s` decrease time between ticks, speeding up game speed

## Pathing
The snake's pathing algorithm is based on randomly generated hamiltonian cycles. At the start of each game, a new random hamiltonian cycle is generated using a randomly weighted undirected graph, prim's algorithm, and a maze following system to translate the resulting glyph into a hamiltonian cycle. This cycle is then used to direct the snake so as to avoid collisions or block-ins. To improve the pathing, sections of the cycle can skipped so long as it moves the head closer to the apple without potentially causing a collission.
#### Hamiltonian Cycle Generation
The algorithm is outlined by Pascal Sommer in his median article: [Generating Hamiltonian Cycles in Rectangular Grid Graphs](https://medium.com/@pascal.sommer.ch/generating-hamiltonian-cycles-in-rectangular-grid-graphs-316c94ecefe0).

The basic idea follows two basic steps.
1. Generate a random maze using prim's algorithm on a random graph. So long as the graph represents a grid and the weights for the edges are well distributed, Prim's algorithm generates a maze with no internal loops.
2. The maze can then be "solved" by following one wall through the entire structure. In this implementation, the right wall is used. This produces a path that outlines the maze and incidentally produces a complete hamiltonian cycle when imprinted on a grid of twice the length and twice the height.

#### Cycle Skipping
To improve the speed of the snake, the cycle can be shortcut when possible. The hamiltonian cycle described above is stored in a matrix of increasing values. These values can be seen as steps withing the cycle. So long as the snake can only skip segments of the cycle in a strictly increasing manner, there is no concern that the head will skip into a loop already bisected by some segment of the body. In addition, this system allows the snake to ensure that it does not skip past the section of the cycle which contains the apple.

#### Attempts with A*
A star is also implemented for testing, it can be enabled using `cargo run astar`. The algorithm is efficient at pathing around the existing structure of the snake, however it lacks many of the strengths of the hamiltonian cycle based system. A* has a tendency to path itself into a corner without realizing and the needed logic to prevent this shortcoming is to complex to be recalculated in real time. For these reasons, the A* algorithm is left strictly as a comparison to its hamiltonian counterpart.

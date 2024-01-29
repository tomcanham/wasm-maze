This application was built as a learning project.

It demonstrates a few concepts:

1. Maze generation using the simplest (recursive backtracking) algorithm. Rust is very competent at tail-recursion algorithms like this, so it is extremely performant. See some alternatives at [Wikipedia](https://en.wikipedia.org/wiki/Maze_generation_algorithm)

2. Pathfinding (in the generated maze) using the A\* pathfinding (algorithm)[https://en.wikipedia.org/wiki/A*_search_algorithm].

3. Rust/WASM interaction. Most of the skeleton of the app was built from and inspired by the Rust WASM "Game of Life" demo (see [here](https://rustwasm.github.io/docs/book/game-of-life/introduction.html))

4. A smidgen of React. Although this demo is not primarily about React, I wanted to demonstrate that I know the fundamentals -- at least enough to build this simple Webpack/React based app showcasing the WASM maze generator/finder.

All graphics are done with the (relatively slow) Canvas 2D drawing API.

Future improvements may (or may not) include:

- Moving rendering to WebGL (fast, faster, fastest!)
- Cleaning up the WASM bindings as much as possible. Right now, every cell is copied between the Rust/WASM context and the JavaScript web app. Obviously this is not scalable for "large" mazes. This dovetails with a general theme of "cleanup and performance."
- Docs! Building the app is complicated (because... WASM is complicated), and I need to set up a proper build system. Maybe I can leverage Cargo (build scripts)[https://doc.rust-lang.org/cargo/reference/build-scripts.html]?
- Making it more interesting. It should be possible -- with some thought, work, and refactoring -- to turn this into a little "PacMan" like game.

LICENSE: You are free to copy, modify, and distribute this source for any purposes, as long as attribution is maintained.

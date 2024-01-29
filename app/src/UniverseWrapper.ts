import { Universe } from "wasm_maze";
import { memory } from "wasm_maze/wasm_maze_bg.wasm";
import { CELL_SIZE, HEIGHT, WIDTH } from "./constants";

export default class UniverseWrapper {
  public pathfinder: any;
  ctx: any;
  universe: Universe;
  width: number;
  height: number;
  start_index: number;
  start_row: number;
  start_col: number;
  end_index: number;
  end_row: number;
  end_col: number;

  constructor(ctx: any) {
    const universe = Universe.new(HEIGHT, WIDTH);
    const width = universe.width();
    const height = universe.height();
    const pathfinder = universe.get_pathfinder();

    const start_index = pathfinder.start_index();
    const [start_row, start_col] = this.index_to_coords(start_index);

    const end_index = pathfinder.end_index();
    const [end_row, end_col] = this.index_to_coords(end_index);

    this.ctx = ctx;
    this.universe = universe;
    this.width = width;
    this.height = height;
    this.pathfinder = pathfinder;
    this.start_index = start_index;
    this.start_row = start_row;
    this.start_col = start_col;
    this.end_index = end_index;
    this.end_row = end_row;
    this.end_col = end_col;
  }

  index_to_coords(index: number): [number, number] {
    const row = Math.floor(index / this.width);
    const col = index % this.width;

    return [row, col];
  }

  getIndex(row: number, column: number): number {
    return row * this.width + column;
  }

  isStart([row, col]: [number, number]): boolean {
    return row === this.start_row && col === this.start_col;
  }

  isEnd([row, col]: [number, number]): boolean {
    return row === this.end_row && col === this.end_col;
  }

  drawMaze() {
    const TOP = 1;
    const BOTTOM = 2;
    const LEFT = 4;
    const RIGHT = 8;

    const ctx = this.ctx;
    ctx.setLineDash([0, 0]);

    const cellsPtr = this.universe.cells();
    const cells = new Uint8Array(
      memory.buffer,
      cellsPtr,
      this.width * this.height
    );

    // Iterate through all the cells and draw them.
    for (let row = 0; row < this.height; row++) {
      for (let col = 0; col < this.width; col++) {
        let coords: [number, number] = [row, col];
        const isStartCell = this.isStart(coords);
        const isEndCell = this.isEnd(coords);

        const idx = this.getIndex(row, col);
        const cell = cells[idx];
        ctx.strokeStyle = "#000000";

        if (cell & TOP) {
          ctx.beginPath();
          ctx.moveTo(col * CELL_SIZE, row * CELL_SIZE);
          ctx.lineTo((col + 1) * CELL_SIZE, row * CELL_SIZE);
          ctx.stroke();
        }

        if (cell & BOTTOM) {
          ctx.beginPath();
          ctx.moveTo(col * CELL_SIZE, (row + 1) * CELL_SIZE);
          ctx.lineTo((col + 1) * CELL_SIZE, (row + 1) * CELL_SIZE);
          ctx.stroke();
        }

        if (cell & LEFT) {
          ctx.beginPath();
          ctx.moveTo(col * CELL_SIZE, row * CELL_SIZE);
          ctx.lineTo(col * CELL_SIZE, (row + 1) * CELL_SIZE);
          ctx.stroke();
        }

        if (cell & RIGHT) {
          ctx.beginPath();
          ctx.moveTo((col + 1) * CELL_SIZE, row * CELL_SIZE);
          ctx.lineTo((col + 1) * CELL_SIZE, (row + 1) * CELL_SIZE);
          ctx.stroke();
        }

        if (isStartCell) {
          ctx.fillStyle = "#FFB0B0";
          ctx.fillRect(
            col * CELL_SIZE + 1,
            row * CELL_SIZE + 1,
            CELL_SIZE - 2,
            CELL_SIZE - 2
          );
        }

        if (isEndCell) {
          ctx.fillStyle = "#B0B0FF";
          ctx.fillRect(
            col * CELL_SIZE + 1,
            row * CELL_SIZE + 1,
            CELL_SIZE - 2,
            CELL_SIZE - 2
          );
        }
      }
    }
  }

  drawPath() {
    const ctx = this.ctx;

    ctx.beginPath();
    ctx.moveTo(
      (this.start_col + 0.5) * CELL_SIZE,
      (this.start_row + 0.5) * CELL_SIZE
    );
    ctx.setLineDash([5, 5]);

    const path = this.pathfinder.path();
    const pathLength = path.length;

    for (let i = 0; i < pathLength; i++) {
      const index = path[i];
      const [row, col] = this.index_to_coords(index);
      ctx.lineTo((col + 0.5) * CELL_SIZE, (row + 0.5) * CELL_SIZE);
    }

    ctx.strokeStyle = "#FF7777";

    ctx.stroke();

    console.log("Optimal path length: ", pathLength);
  }
}

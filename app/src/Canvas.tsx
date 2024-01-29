import UniverseWrapper from "./UniverseWrapper";
import { useEffect, useRef } from "react";
import { CELL_SIZE, HEIGHT, WIDTH } from "./constants";

const Canvas = (props: any) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas) {
      const context = canvas.getContext("2d");
      if (context) {
        context.clearRect(0, 0, canvas.width, canvas.height);
        const world = new UniverseWrapper(context);

        world.drawMaze();

        let count = 0;
        while (!world.pathfinder.done()) {
          count++;
          world.pathfinder.tick();
        }
        world.drawPath();
        console.log(`Solution took ${count} ticks`);
      }
    }
  }, []);

  return (
    <canvas
      id="game-canvas"
      ref={canvasRef}
      height={(CELL_SIZE + 1) * HEIGHT + 1}
      width={(CELL_SIZE + 1) * WIDTH + 1}
      {...props}
    />
  );
};

export default Canvas;

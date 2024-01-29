import React, { useEffect, useRef } from "react";
import { createRoot } from "react-dom/client";
import Canvas from "./Canvas";

const rootElement = document.getElementById("app");
if (rootElement) {
  const root = createRoot(rootElement);

  root.render(
    <React.StrictMode>
      <h1>Pathfinding Demo</h1>
      <Canvas />
    </React.StrictMode>
  );
}

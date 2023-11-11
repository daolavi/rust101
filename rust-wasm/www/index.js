import { Universe } from "rust-wasm/rust_wasm";

const pre = document.getElementById("rust-wasm");
const universe = Universe.new();
const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();
  requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);

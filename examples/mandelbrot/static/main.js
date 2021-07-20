import init, { start } from './pkg/mandelbrot.js';
async function main() {
   await init('./pkg/mandelbrot_bg.wasm');
   start();
}
main()


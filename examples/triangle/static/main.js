import init, { start } from './pkg/triangle.js';
async function main() {
   await init('./pkg/triangle_bg.wasm');
   start();
}
main()


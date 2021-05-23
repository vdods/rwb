import init, { run_app } from './pkg/simpleapp.js';
async function main() {
   await init('./pkg/simpleapp_bg.wasm');
   run_app();
}
main()

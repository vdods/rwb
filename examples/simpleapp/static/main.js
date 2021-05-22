import init, { run_app } from './pkg/simpleapp.js';
async function main() {
   await init('/simpleapp/pkg/simpleapp_bg.wasm');
   run_app();
}
main()


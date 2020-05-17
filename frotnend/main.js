import init, { run_app } from './pkg/webapp.js';
async function main() {
   await init('./webapp_bg.wasm');
   run_app();
}
main()
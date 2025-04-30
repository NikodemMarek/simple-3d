import init from "./pkg/simple_3d_wasm.js";

async function run() {
	await init();
	console.log("WASM module initialized and Rust code executed.");
}

run();

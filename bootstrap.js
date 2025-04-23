import init from "./pkg/simple_3d.js";

async function run() {
	await init();
	console.log("WASM module initialized and Rust code executed.");
}

run();

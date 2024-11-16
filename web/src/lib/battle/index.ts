import { Battle } from 'machine';

export function play(wasm_bytes_1: Uint8Array, wasm_bytes_2: Uint8Array) {
	const battle = new Battle();
	battle.add_bot(wasm_bytes_1);
	battle.add_bot(wasm_bytes_2);
	const result = battle.execute();
	console.log(result);
}

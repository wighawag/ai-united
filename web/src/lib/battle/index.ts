import { Battle } from 'machine';
import { writable } from 'svelte/store';

const MAX_NUM_UPDATES = 1_000_000; // TODO share with machine

export type Position = { x: number; y: number };

export type BattleState = {
	step: number;
	battle?: Battle;
	bot1: Position;
	bot2: Position;
	ball: Position;
	initialized: boolean;
};
const $battle: BattleState = {
	step: 0,
	bot1: { x: 0, y: 0 },
	bot2: { x: 0, y: 0 },
	ball: { x: 0, y: 0 },
	initialized: false
};
const _battle = writable($battle);

export const battle = {
	subscribe: _battle.subscribe
};

function updateState() {
	_battle.set($battle);
}

let currentPendingAnimationFrame: number;
function update() {
	if ($battle.battle) {
		const result = $battle.battle.update();
		if (result > 0) {
			updateState();
			console.log(`Winner: ${result}`);
			return;
		} else {
			$battle.step++;
			if ($battle.step > MAX_NUM_UPDATES) {
				updateState();
				console.log(`DRAW: ${$battle.step} updates executed.`);
				return;
			}
		}

		updateState();
		currentPendingAnimationFrame = requestAnimationFrame(update);
	}
}

export function play(wasm_bytes_1: Uint8Array, wasm_bytes_2: Uint8Array) {
	const battle = new Battle();
	$battle.battle = battle;

	battle.add_bot(wasm_bytes_1);
	battle.add_bot(wasm_bytes_2);
	battle.init();

	update();
}

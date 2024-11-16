import { Battle } from 'machine';
import { writable } from 'svelte/store';

const MAX_NUM_UPDATES = 1_000_000; // TODO share with machine

export type Position = { x: number; y: number; z: number };

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
	bot1: { x: 0, y: 0, z: 0 },
	bot2: { x: 0, y: 0, z: 0 },
	ball: { x: 0, y: 0, z: 0 },
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

		const bot1Postion = $battle.battle.getBot1();
		$battle.bot1.x = bot1Postion.x;
		$battle.bot1.y = bot1Postion.y;
		$battle.bot1.z = bot1Postion.z;

		const bot2Postion = $battle.battle.getBot2();
		$battle.bot2.x = bot2Postion.x;
		$battle.bot2.y = bot2Postion.y;
		$battle.bot2.z = bot2Postion.z;

		const ballPostion = $battle.battle.getBall();
		$battle.ball.x = ballPostion.x;
		$battle.ball.y = ballPostion.y;
		$battle.ball.z = ballPostion.z;

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

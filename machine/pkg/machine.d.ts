/* tslint:disable */
/* eslint-disable */
export function start(): void;
export class Battle {
  free(): void;
  constructor();
  /**
   * @returns {Position}
   */
  get_bot1(): Position;
  /**
   * @returns {Position}
   */
  get_bot2(): Position;
  /**
   * @returns {Position}
   */
  get_ball(): Position;
  /**
   * @param {Uint8Array} wasm_bytes
   */
  add_bot(wasm_bytes: Uint8Array): void;
  init(): void;
  /**
   * @returns {number}
   */
  update(): number;
  /**
   * @returns {number}
   */
  execute(): number;
}
export class Position {
  free(): void;
  x: number;
  y: number;
  z: number;
}

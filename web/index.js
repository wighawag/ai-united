import { Executor } from "../machine/pkg/machine.js";

const fromHexString = (hexString) =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

async function setup() {
  const payload = "20";

  const bytes = fromHexString(payload);
  console.log(bytes);
  const executor = new Executor();
  const result = executor.execute(bytes);
  console.log(result);
}

setup();

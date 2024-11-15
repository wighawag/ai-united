# botpit

## dependencies

Required:

- cargo / rust
- pnpm
- docker

Optional:

- cargo-watch

  ```bash
  cargo install cargo-watch
  ```

- zellij (for running all dev processes)

  https://zellij.dev/

## setup

```bash
pnpm i
```

## dev

```bash
pnpm start
```

This will run zellij and execute the multiple processes to build the various binaries and watch for changes

## test a bot

### build the cartesi machine

(already built via pnpm start)

```bash
pnpm cartesi build
```

### run it

```bash
pnpm cartesi run
```

### transform bot wasm to hex

```bash
cd wasm2hex
target/debug/wasm2hex --input ../bots/rust/target/wasm32-unknown-unknown/release/bot_rust.wasm --output bot.json
```

### execute the machine with the bot module as input

```bash
pnpm cartesi send generic
# (hex encoding option)
# copy from the result of `wasm2hex`
```

### you can also execue natively with `machine-executor`

```bash
cd machine-executor
target/debug/machine-executor --bot <hex string | path to json>
# example: target/debug/machine-executor --bot ../wasm2hex/bot.json
```

## manual builds

### build the rust bot

```bash
cd bots/rust # you need to be in the bots/rust folder
cargo build --release
```

### build the wasm2hex cli

```bash
cd wasm2hex # you need to be in the wasm2hex folder
cargo build
```

### build the `machine-executor`

it can execute natively the bots

```bash
cd machine-executor # you need to be in the machine-executor folder
cargo build
```

## you can also run it in browser

### build the machine wasm

```bash
cd machine # you need to be in the machine folder
wasm-pack build --target web && echo "export const wasmExports = await __wbg_init();" >> pkg/machine.js
```

### use it in web

```bash
npx serve .
# navigate to http://localhost:3000
```

# template: cartesi + rust + web

## dependencies

Required:

- cargo / rust
- pnpm
- docker

### Optional but assumed for this readme:

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

## test

### build the cartesi machine

(already built via pnpm start)

```bash
pnpm cartesi build
```

### run it

```bash
pnpm cartesi run
```

### execute the machine with the module as input

```bash
pnpm cartesi send generic
# use a space character as input and you ll'get 0x20/32 as output
```

### run it in browser

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

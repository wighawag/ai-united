# botpit

generate hex for bot

```bash
cd wasm2hex
target/debug/wasm2hex --input ../bots/rust/target/wasm32-unknown-unknown/release/bot_rust.wasm --output bot.json
```

execute via standalone machine-executor

```bash
target/debug/machine-executor --bot ../wasm2hex/bot.json
```

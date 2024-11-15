# botpit

generate hex for module

```bash
cd wasm2hex
target/debug/wasm2hex --input ../modules/rust/target/wasm32-unknown-unknown/release/module_rust.wasm --output module.json
```

execute via standalone machine-executor

```bash
target/debug/machine-executor --module ../wasm2hex/module.json
```

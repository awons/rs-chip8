# Chip8 emulator

## Three flavors of emulator

### Console

```bash
cargo build --release
./target/release/console [path/to/game/rom]
```

WIP
* keyboard does not behave as intended

### WebAssembly

```bash
cd wasm-app && wasm-pack build
cd ../chip8-www
npm install
npm start
```

now go to the browser under `http://localhost:8080`.

WIP
* only display works

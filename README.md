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
cd wasm-app && wasm-pack build --release
cd ../chip8-www
npm install
npm start
```

now go to the browser under `http://localhost:8080`.

Keyboard:
1234 -> 123C
QWER -> 456D
ASDF -> 789E
ZXCV -> A0BF

Esc -> break game

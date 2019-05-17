// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-app/wasm_app_bg";
import { Game } from "wasm-app";


const romInput = document.getElementById("rom-file");
const startButton = document.getElementById("start-game");
const romReader = new FileReader();

let game = null;
let romBytes = null;
let globalReloadFlag = null;
let pressedKey = null;

const sleep = (milliseconds) => {
    return new Promise(resolve => setTimeout(resolve, milliseconds))
}

const runGame = async (runningGame) => {
    const localReloadFlag = globalReloadFlag = new Object();

    const pressedKeyPtr = runningGame.get_pressed_key_ptr();
    pressedKey = new Uint8Array(memory.buffer, pressedKeyPtr, 0x1);

    while (runningGame.run_cycle()) {
        if (localReloadFlag !== globalReloadFlag) {
            return;
        }
        await sleep(2);
    };
}

romInput.addEventListener("change", event => {
    romReader.readAsArrayBuffer(romInput.files[0]);
})

romReader.addEventListener("load", event => {
    game = Game.new();
    const romPtr = game.get_rom_ptr();
    romBytes = new Uint8Array(memory.buffer, romPtr, 0xe00);
    romBytes.set(new Uint8Array(romReader.result));
});

startButton.addEventListener("click", event => {
    globalReloadFlag = new Object();
    runGame(game.start());
});

document.addEventListener("keydown", event => {
    pressedKey.set([event.keyCode]);
});

document.addEventListener("keyup", event => {
    pressedKey.set([0]);
})

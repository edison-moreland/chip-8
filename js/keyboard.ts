// Keyboard allows querying which Chip-8 key is currently pressed
// https://codepen.io/edison-moreland/pen/PowOGqO
type Chip8Key = -1|0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15

export class Keyboard {
    keyMap: Map<string, Chip8Key>;
    keyPressed: Chip8Key;

    constructor() {
        // -1 = No key pressed
        this.keyPressed = -1;

        // Maps physical keys to the chip-8's hex keyboard (0-15)
        this.keyMap = new Map([
            ["Digit1", 1], ["Digit2", 2], ["Digit3", 3], ["Digit4", 12],
            ["KeyQ", 4], ["KeyW", 5], ["KeyE", 6], ["KeyR", 13],
            ["KeyA", 7], ["KeyS", 8], ["KeyD", 9], ["KeyF", 14],
            ["KeyZ", 10], ["KeyX", 0], ["KeyC", 11], ["KeyV", 15],
        ]);
        window.addEventListener("keydown", this);
        window.addEventListener("keyup", this);
    }

    getKey(): Chip8Key {
        return this.keyPressed;
    }

    handleEvent(event: KeyboardEvent) {
        // The two event handlers were combined so that 
        // "Keyboard" implements the EventListener interface,
        // which allows access to "this" while handling
        // events. Other wise, we would only have access 
        // to the global scope.
        // https://developer.mozilla.org/en-US/docs/Web/API/EventListener

        const key = this.keyMap.get(event.code);

        // Hop out if key was not a chip-8 key
        if (key === undefined) { return }

        switch (event.type) {
            case "keydown":
                // Only change key if a key isn't already pressed
                if (this.keyPressed === -1) {
                    this.keyPressed = key
                }
                break;

            case "keyup":
                // Only change if the lifted key is the 
                // same key that was already pressed
                if (this.keyPressed === key) {
                    this.keyPressed = -1;
                }
                break;
        }
    }
}
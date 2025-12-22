
const KEYBOARD_LAYOUT = [
    "QWERTYUIOP",
    ".ASDFGHJKL.",
    "%ZXCVBNM!"
]


function PopulateKeyboard(onPressCallback, onEnter, onBackspace) {
    let keyboard_container = document.getElementById("keyboard");

    for (line of KEYBOARD_LAYOUT) {
        let lineElement = document.createElement("div");
        lineElement.classList.add("keyboard-line");
        keyboard_container.appendChild(lineElement);

        for (let i = 0; i < line.length; i++) {
            let el = document.createElement("div");
            lineElement.appendChild(el);
            let label = document.createElement("span");
            el.appendChild(label);

            if (line[i] == '.') {
                el.classList.add("keyboard-void");
            }
            else if (line[i] == '%') {
                el.classList.add("keyboard-enter");
                label.textContent = "ENTER"
                el.addEventListener("mouseup", ev => onEnter())
            }
            else if (line[i] == '!') {
                el.classList.add("keyboard-backspace");
                label.textContent = "BACKSPACE"
                el.addEventListener("mouseup", ev => onBackspace())
            }
            else {
                el.classList.add("keyboard-letter");
                label.textContent = line[i];

                let char = line[i];
                el.addEventListener("mouseup", ev => onPressCallback(char))
            }
        }
    }
}


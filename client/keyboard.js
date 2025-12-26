
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
                el.id = "key-enter";
            }
            else if (line[i] == '!') {
                el.classList.add("keyboard-backspace");
                label.textContent = "BACKSPACE"
                el.addEventListener("mouseup", ev => onBackspace())
                el.id = "key-backspace";
            }
            else {
                el.classList.add("keyboard-letter");
                label.textContent = line[i];

                let char = line[i];
                el.addEventListener("mouseup", ev => onPressCallback(char))
                el.id = "key-" + char;
            }
        }
    }
}

function SetKeyboardHint(char, hintType) {
    let el = document.getElementById("key-" + char);

    el.classList.remove("hint-gray")
    el.classList.remove("hint-green")
    el.classList.remove("hint-yellow")
    el.classList.remove("hint-red")
    
    if (hintType == HINT_GRAY) {
        el.classList.add("hint-gray")
    }
    else if (hintType == HINT_GREEN) {
        el.classList.add("hint-green")
    }
    else if (hintType == HINT_YELLOW) {
        el.classList.add("hint-yellow")
    }
    else if (hintType == HINT_RED) {
        el.classList.add("hint-red")
    }
    else if (hintType == HINT_NONE) {
        // Nothing
    }
    else {
        console.error("Uuh?")
    }
}


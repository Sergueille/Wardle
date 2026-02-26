
const KEYBOARD_LAYOUT_QWERTY = [
    "QWERTYUIOP",
    ".ASDFGHJKL.",
    "%ZXCVBNM!"
];

const KEYBOARD_LAYOUT_AZERTY = [
    "AZERTYUIOP",
    "QSDFGHJKLM",
    ".%WXCVBN!."
];


function PopulateKeyboard(onPressCallback, onEnter, onBackspace, language) {
    let keyboardContainer = document.getElementById("keyboard");
    keyboardContainer.innerHTML = ""; // Clear the parent

    let layout;
    if (language == "English") {
        layout = KEYBOARD_LAYOUT_QWERTY;
    }
    else if (language == "French") {
        layout = KEYBOARD_LAYOUT_AZERTY;
    }

    for (line of layout) {
        let lineElement = document.createElement("div");
        lineElement.classList.add("keyboard-line");
        keyboardContainer.appendChild(lineElement);

        for (let i = 0; i < line.length; i++) {
            let el = document.createElement("div");
            let label = document.createElement("span");
            el.appendChild(label);

            if (line[i] == '.') {
                el.classList.add("keyboard-void");
                lineElement.appendChild(el);
            }
            else if (line[i] == '%') {
                let key = document.getElementById("key-enter-template").cloneNode(true);
                key.id = "key-enter";
                key.addEventListener("mouseup", ev => onEnter());
                lineElement.appendChild(key);
            }
            else if (line[i] == '!') {
                let key = document.getElementById("key-backspace-template").cloneNode(true);
                key.id = "key-backspace";
                key.addEventListener("mouseup", ev => onBackspace());
                lineElement.appendChild(key);
            }
            else {
                el.classList.add("keyboard-letter");
                label.textContent = line[i];

                let char = line[i];
                el.addEventListener("mouseup", ev => onPressCallback(char))
                el.id = "key-" + char;

                lineElement.appendChild(el);
            }
        }
    }

    // Set initial icons
    SetSubElement("key-enter", "icon-enter");
    SetSubElement("key-backspace", "icon-backspace");
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

function ClearKeyboardHints() {
    let keyboard_container = document.getElementById("keyboard");
    for (let line of keyboard_container.children) {
        for (let key of line.children) {
            key.classList.remove("hint-gray")
            key.classList.remove("hint-green")
            key.classList.remove("hint-yellow")
            key.classList.remove("hint-red")
        }
    }
}

function SetKeyboardEnterIcon(icon_id) {
    SetSubElement("key-enter", icon_id)
}

function SetKeyboardBackspaceIcon(icon_id) {
    SetSubElement("key-backspace", icon_id)
}


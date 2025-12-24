
const HINT_REVEAL_ANIMATION_DURATION = 100; // ms
const MULTIPLE_LETTERS_ANIMATION_DELAY = 40; // ms
const WIN_ANIMATION_DELAY = 100; // ms


function PopulateWordGrids(nbColumn, nbRow, onSabotage) {
    let left = document.getElementById("left-grid");
    let right = document.getElementById("right-grid");

    for (let i = 0; i < 2; i++) {
        for (let y = 0; y < nbRow; y++) {
            for (let x = 0; x < nbColumn; x++) {
                let id = GetCellId(i == 0, x, y);

                let container = document.createElement("div");
                container.classList.add("word-grid-cell-container");
                container.id = id + "-container";

                let el = document.createElement("div");
                el.classList.add("word-grid-cell");
                el.classList.add("empty");
                el.id = id;

                let label = document.createElement("span");
                label.classList.add("word-grid-label");
                label.id = id + "-label";

                [left, right][i].appendChild(container);
                container.appendChild(el);
                el.appendChild(label);

                el.addEventListener("mouseup", () => {
                    onSabotage(x, y);
                })
            }
        }
    }
}

function SetLetter(isLeftGrid, x, y, letter) {
    let cell = GetCell(isLeftGrid, x, y);
    let label = GetCellLabel(isLeftGrid, x, y);

    cell.classList.add("animate-type");
    cell.classList.remove("empty");
    cell.classList.add("typed-unchecked");

    label.textContent = letter;
}

function RemoveLetter(isLeftGrid, x, y,) {
    let cell = GetCell(isLeftGrid, x, y);
    let label = GetCellLabel(isLeftGrid, x, y);

    cell.classList.remove("animate-type");
    cell.classList.remove("typed-unchecked");
    cell.classList.add("empty");

    label.textContent = "";
}

function SetHint(isLeftGrid, x, y, hintColor) {
    let cell = GetCell(isLeftGrid, x, y);
    let label = GetCellLabel(isLeftGrid, x, y);

    let hintClassName;
    if (hintColor == 0) {
        hintClassName = "hint-green";
    }
    else if (hintColor == 1) {
        hintClassName = "hint-yellow";
    }
    else if (hintColor == 2) {
        hintClassName = "hint-red";
    }
    else if (hintColor == 3) {
        hintClassName = "hint-none";
    }
    else {
        hintClassName = "hint-green";
        console.error("Unknown hintColor value given!");
    }

    cell.classList.add("animate-hint");

    setTimeout(() => {
        cell.classList.remove("empty");
        cell.classList.remove("typed-unchecked");
        cell.classList.add(hintClassName);
    }, HINT_REVEAL_ANIMATION_DURATION)
}

function SetWord(isLeftGrid, y, word) {
    let setLetter = i => {
        if (i == WORD_LENGTH) { return; }
        SetLetter(isLeftGrid, i, y, word[i]);

        setTimeout(() => setLetter(i+1), MULTIPLE_LETTERS_ANIMATION_DELAY)
    }

    setLetter(0);
}

function SetHints(isLeftGrid, y, hints) {
    let setHint = i => {
        if (i == WORD_LENGTH) { return; }
        SetHint(isLeftGrid, i, y, hints[i]);

        setTimeout(() => setHint(i+1), MULTIPLE_LETTERS_ANIMATION_DELAY)
    }

    setHint(0);
}

function SetSabotageTarget(isLeftGrid, rowId, enable) {
    for (let y = 0; y < MAX_WORD_COUNT; y++) {
        for (let x = 0; x < WORD_LENGTH; x++) {
            let el = document.getElementById(GetCellId(isLeftGrid, x, y));

            if (!enable || y != rowId) {
                el.classList.remove("sabotage-target")
            }
            else {
                el.classList.add("sabotage-target")
            }
        }
    }
}

function InvalidAnimation(isLeftGrid, rowId) {
    for (let y = 0; y < MAX_WORD_COUNT; y++) {
        for (let x = 0; x < WORD_LENGTH; x++) {
            let el = document.getElementById(GetCellId(isLeftGrid, x, y));

            if (y == rowId) {
                el.classList.add("invalid-animation")
                el.classList.remove("animate-type")
                
                setTimeout(() => el.classList.remove("invalid-animation"), 300);
            }
        }
    }

    console.log(rowId);
}

function WinAnimation(isLeftGrid, rowId) {
    for (let y = 0; y < MAX_WORD_COUNT; y++) {
        for (let x = 0; x < WORD_LENGTH; x++) {
            let el = document.getElementById(GetCellId(isLeftGrid, x, y));

            if (y == rowId) {
                setTimeout(() => {
                    el.classList.add("win-animation");
                    el.classList.add("hint-green")
                    el.classList.remove("typed-unchecked");
                }, WIN_ANIMATION_DELAY * x);
            }
        }
    }

    console.log(rowId);
}

function SetLeftGridActive() {
    document.getElementById("left-grid").classList.remove("inactive");
    document.getElementById("right-grid").classList.add("inactive");
}

function SetRightGridActive() {
    document.getElementById("left-grid").classList.add("inactive");
    document.getElementById("right-grid").classList.remove("inactive");
}

function SetBothGridInactive() {
    document.getElementById("left-grid").classList.add("inactive");
    document.getElementById("right-grid").classList.add("inactive");
}

function SetBothGridActive() {
    document.getElementById("left-grid").classList.remove("inactive");
    document.getElementById("right-grid").classList.remove("inactive");
}

function GetCell(isLeftGrid, x, y) {
    return document.getElementById(GetCellId(isLeftGrid, x, y));
}

function GetCellLabel(isLeftGrid, x, y) {
    return document.getElementById(GetCellId(isLeftGrid, x, y) + "-label");
}

function GetCellId(isLeftGrid, x, y) {
    return "word-grid-cell-" + (isLeftGrid ? 0 : 1) + "-" + x + "-" + y;
}

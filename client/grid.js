
const HINT_REVEAL_ANIMATION_DURATION = 100; // ms


function PopulateWordGrids(nbColumn, nbRow) {
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

function GetCell(isLeftGrid, x, y) {
    return document.getElementById(GetCellId(isLeftGrid, x, y));
}

function GetCellLabel(isLeftGrid, x, y) {
    return document.getElementById(GetCellId(isLeftGrid, x, y) + "-label");
}

function GetCellId(isLeftGrid, x, y) {
    return "word-grid-cell-" + (isLeftGrid ? 0 : 1) + "-" + x + "-" + y;
}

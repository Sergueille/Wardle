
const WORD_LENGTH = 6;
const MAX_WORD_COUNT = 1;
windowFocused = true;

PopulateWordGrids(6, 1, () => {});



setTimeout(() => SetWord(true, 0, "WARDLE"), 500);
setTimeout(() => SetHints(true, 0, [3, 0, 3, 3, 3, 3]), 800);

let boom = document.getElementById("boom").cloneNode();


setTimeout(() => {
    document.getElementById("word-grid-cell-0-1-0").appendChild(boom);
    boom.classList.remove("hidden");
    boom.classList.add("transparent");
    boom.setAttribute("src", "asymptote_verticale_transparent.gif")
}, 750);

setTimeout(() => {
    document.getElementById("word-grid-cell-0-1-0").appendChild(boom);
    boom.classList.remove("transparent");
    boom.setAttribute("src", "asymptote_verticale_transparent.gif")
}, 1500);

setTimeout(() => SetHint(true, 1, 0, 2), 1600);


setTimeout(() => {
    boom.classList.add("hidden");
}, 3300);

//setTimeout(() => WinAnimation(true, 0), 1800);





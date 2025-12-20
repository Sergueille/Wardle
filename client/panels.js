

let currentPanel = null;

function HideAllPanels() {
    document.querySelectorAll(".panel").forEach(panel => {
        panel.classList.add("hidden");
    })
}


function ShowPanel(id) {
    HideAllPanels();

    currentPanel = document.getElementById(id);
    currentPanel.classList.remove("hidden");
}

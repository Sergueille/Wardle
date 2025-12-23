
const TOAST_DURATION = 5000;

let toastHideHandle = undefined;

function Toast(toastId) {
    let container = document.getElementById("toasts-container");

    for (let element of container.children) {
        if (element.id == toastId) {
            element.classList.remove("hidden");
        }
        else {
            element.classList.add("hidden");
        }
    }

    container.classList.add("visible");

    if (toastHideHandle) { 
        clearTimeout(toastHideHandle);
        toastHideHandle = undefined;
    }

    toastHideHandle = setTimeout(() => {
        container.classList.remove("visible");
    }, TOAST_DURATION);
}


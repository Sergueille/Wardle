
const TOAST_DURATION = 5000;

let toastHideHandle = undefined;

function SetupToasts() {
    let container = document.getElementById("toasts-container");
    for (let element of container.children) {
        element.classList.add("hidden");
    }
}

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

function CustomToast(contents) {
    document.getElementById("toast-test").innerHTML = contents;
    Toast("toast-test");
}


// Will add `sub-hidden` class to all children of element with id `parent_id`, except the one with the specified ids
function SetSubElement(parent_id, id) {
    let container = document.getElementById(parent_id);

    for (let element of container.children) {
        if (element.id == id) {
            element.classList.remove("sub-hidden")
        }
        else {
            element.classList.add("sub-hidden")
        }
    }
}

// Will add `sub-hidden` class to all children of element with id `parent_id`
function HideChildren(parent_id) {
    let container = document.getElementById(parent_id);

    for (let element of container.children) {
        element.classList.add("sub-hidden")
    }
}



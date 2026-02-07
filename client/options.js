
const OPTIONS_UI = {
    timer: {
        displayName: "Timer",
        possibleValues: [{
                val: 0,
                name: "Disabled",
            },{
                val: 5,
                name: "5 seconds",
            },{
                val: 10,
                name: "10 seconds",
            },{
                val: 15,
                name: "15 seconds",
            },{
                val: 30,
                name: "30 seconds",
            },
        ],
    }
}

// This initial value is the default value for the options
let currentOptions = {
    timer: 0.0
};

// Populates the UI with the current options
function InitializeOptions()
{
    let parent = document.getElementById("options-container");

    for (let [key, val] of Object.entries(OPTIONS_UI)) {
        let cont = document.createElement("div");
        cont.classList.add("option-row");
        
        let title = document.createElement("span");
        title.classList.add("option-name");
        title.textContent = `${val.displayName}: `;

        let sel = document.createElement("select");
        sel.classList.add("option-select");

        for (let i = 0; i < val.possibleValues.length; i++) {
            let option = document.createElement("option");
            option.textContent = val.possibleValues[i].name;
            option.setAttribute("value", i);
            sel.appendChild(option);
        }

        sel.addEventListener("change", ev => {
            console.log("Option changed");
            currentOptions[key] = val.possibleValues[ev.target.value].val;
            OnOptionsChanged();
        });

        let currentVal = currentOptions[key];
        let foundAVal = false;
        for (let i = 0; i < val.possibleValues.length; i++) {
            if (val.possibleValues[i].val == currentVal) {
                sel.value = i;
                foundAVal = true;
            }
        }

        if (!foundAVal) {
            console.error(`Invalid value found for option "${key}"`)
            sel.value = 0;
            currentOptions = val.possibleValues[0].val;
            OnOptionsChanged();
        }

        cont.appendChild(title);
        cont.appendChild(sel);

        parent.appendChild(cont);
    }
}

function OnOptionsChanged()
{
    SendOptionsToServer();
    StoreOptions();
}

function StoreOptions()
{
    window.localStorage.setItem("game-options", JSON.stringify(currentOptions));
}

function LoadOptions()
{
    let tmp = window.localStorage.getItem("game-options");
    if (tmp != null)
    {
        currentOptions = JSON.parse(tmp);
    }
}

function SendOptionsToServer()
{
    TrySendMessage("game-options", {
        "options": currentOptions
    });
}

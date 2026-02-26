
const OPTIONS_UI = {
    language: {
        displayName: "Language",
        possibleValues: [{
                val: "English",
                name: "English",
            },{
                val: "French",
                name: "Français",
            },
        ],
        description: "Language of the word to guess for this game"
    },
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
        description: "The timer will start when the other player takes an action"
    },
}

let defaultOptions = {
    timer: 0.0,
    language: "English"
};

let currentOptions = defaultOptions;
let currentOptionsWereSetByPlayer = false;

function PopulateOptionsInParent(parent, readonly)
{
    parent.innerHTML = ""; // Empty the parent
    parent.removeAttribute("disabled");
    
    for (let [key, val] of Object.entries(OPTIONS_UI)) {
        let cont = document.createElement("div");
        cont.classList.add("option-row");
        parent.appendChild(cont);
        
        let title = document.createElement("span");
        title.classList.add("option-name");
        title.textContent = `${val.displayName}: `;
        cont.appendChild(title);


        let currentVal = currentOptions[key];
        let valueId = null;
        for (let i = 0; i < val.possibleValues.length; i++) {
            if (val.possibleValues[i].val == currentVal) {
                valueId = i;
            }
        }

        if (valueId == null) {
            console.error(`Invalid value found for option "${key}"`)
            sel.value = 0;
            currentOptions = defaultOptions;
            StoreOptions();
            PopulateOptionsInParent(parent, readonly);
            return;
        }

        if (readonly) {
            let valueText = document.createElement("span");
            valueText.classList.add("option-value-text");
            valueText.textContent = val.possibleValues[valueId].name;
            cont.appendChild(valueText);
        }
        else {        
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

            sel.value = valueId;

            cont.appendChild(sel);
        }

        if (val.description != "" && !readonly) {
            let description = document.createElement("span");
            description.classList.add("option-description");
            description.textContent = val.description;
            parent.appendChild(description);
        }
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


const API_URL = "localhost:4268";

const LOAD_DELAY = 700; //ms
const TEMPORARY_INFO_DELAY = 10000; //ms

document.getElementById("join-room-btn").addEventListener("click", ev => JoinRoom())
document.getElementById("create-room-btn").addEventListener("click", ev => CreateRoom())

HideAllPanels();
ShowPanel("start-panel");

function GetInitialGameState() {
    return {
        websocket_connection: null,
        room_code: undefined
    };
}

function JoinRoom()
{
    let code = document.getElementById("join-room-code").value.toLowerCase().trim();

    if (code.length < 8) {
        // TODO
    }
    else {
        console.log("http://" + API_URL + "/join-room/" + code);
        let connection = new WebSocket("http://" + API_URL + "/join-room/" + code);

        document.getElementById("join-room-btn").classList.add("connecting");

        let state = GetInitialGameState();
        state.websocket_connection = connection;

        connection.addEventListener("message", ev => HandleConnectionMessage(state, ev.data));
        connection.addEventListener("open", ev => {
            OnGameStart(state);
        });
        connection.addEventListener("error", ev => {
            document.getElementById("join-room-btn").classList.remove("connecting");
            document.getElementById("join-error").classList.remove("hidden");
        });
    }
}

function CreateRoom()
{
    let connection = new WebSocket("http://" + API_URL + "/create-room");

    let state = GetInitialGameState();
    state.websocket_connection = connection;

    document.getElementById("create-room-btn").classList.add("connecting");

    connection.addEventListener("message", ev => HandleConnectionMessage(state, ev.data));
    connection.addEventListener("error", ev => {
        document.getElementById("create-room-btn").classList.remove("connecting");
        document.getElementById("create-error").classList.remove("hidden");
    });
}

// Called once the game can properly start (both players connected)
function OnGameStart(state) {
    ShowPanel("game-panel");

    // TODO
}

function TrySendMessage(state, msgType, msgContent) {
    if (state.websocket_connection && state.websocket_connection.readyState == WebSocket.OPEN) {
        state.websocket_connection.send(JSON.stringify({
            type: msgType,
            content: msgContent,
        }));
    }
    else {
        console.log("Couldn't send websocket message")
        // TODO: try to reconnect
    }
}

function HandleConnectionMessage(state, msgText) {
    let msg;
    try {
        msg = JSON.parse(msgText);
    }
    catch {
        console.error("Invalid JSON format for websocket message:" + msgText)
    }

    if (msg.type == "room-code") {
        state.roomCode = msg.content;
        document.getElementById("room-code").textContent = state.roomCode;
        navigator.clipboard.writeText(state.roomCode);
        ShowPanel("wait-panel");
    }
    else if (msg.type == "other-player-connected") {
        OnGameStart(state);
    }
    else {
        console.error("Unknown message type: " + msg.type);
    }
}



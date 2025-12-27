
const DEFAULT_API_URL = "[2a09:6847:fa10:1410::278]";
const API_PORT = 4268;

const TEMPORARY_INFO_DELAY = 10000; //ms
const WORD_OK_DELAY = 100; //ms

const PING_SEND_DELAY = 1000; //ms
const RECONNECTION_DELAY = 2000; //ms

const WORD_LENGTH = 5;
const MAX_WORD_COUNT = 6;

const HINT_GREEN = 0;
const HINT_YELLOW = 1;
const HINT_RED = 2;
const HINT_GRAY = 3;
const HINT_NONE = 4;

const PHASE_TYPE = 0;
const PHASE_TYPE_WAIT = 1;
const PHASE_SABOTAGE = 2;
const PHASE_SABOTAGE_WAIT = 3;

document.getElementById("join-room-btn").addEventListener("click", ev => JoinRoom());
document.getElementById("create-room-btn").addEventListener("click", ev => CreateRoom());

document.getElementById("server-url-input").addEventListener("change", ev => SetApiUrl(ev.target.value));
document.getElementById("server-url-input").value = GetApiUrlWithoutPort();

document.addEventListener("keydown", (event) => {
    if (!state || !state.gameStarted) {
        return;
    }

    if (event.code == "Enter") {
        OnEnter();
    }
    else if (event.code == "Backspace") {
        OnBackspace();
    }
    else {
        let letter = event.key.toUpperCase();
        const alphabet = "AZERTYUIOPQSDFGHJKLMWXCVBN";

        if (alphabet.includes(letter)) {
            OnLetterTyped(letter);
        }
    }
});

let state; // Global game state
let allowedWords = [];

function Start() {
    HideAllPanels();
    SetupToasts();
    ShowPanel("start-panel");
    GetAllWords();
}

Start();

function ResetGlobalState() {
    state = {
        // Utility for server interaction
        websocketConnection: null,
        roomCode: undefined,
        pingLoopHandle: undefined,
        gameStarted: false,
        isHostPlayer: false,
        messagesToSend: [], // Message that are sent when not connected are stored here, to send them on reconnection
        lastSentMessage: null, // Also to be resent if there is a problem
        inReconnectionDelay: false,

        // Game state
        playerWords: [],
        enemyWords: [],
        currentTurn: -1,
        currentPhase: PHASE_TYPE,
        knownHints: {},

        // Information for type phase
        typedWord: "",
        waitPhaseInterfaceTimeout: null,
    };
}

function JoinRoom()
{
    ResetGlobalState();

    let code = document.getElementById("join-room-code").value.toLowerCase().trim();
    state.roomCode = code;

    console.log("http://" + GetApiUrl() + "/join-room/" + code);
    let connection = new WebSocket("http://" + GetApiUrl() + "/join-room/" + code);

    document.getElementById("join-room-btn").classList.add("connecting");

    state.websocketConnection = connection;
    state.isHostPlayer = false;

    connection.addEventListener("message", ev => HandleConnectionMessage(ev.data));
    connection.addEventListener("open", ev => {
        StartPingLoop();
        OnGameStart();
    });
    connection.onerror = ev => {
        document.getElementById("join-room-btn").classList.remove("connecting");
        Toast("room-join-failed");
    };
}

function CreateRoom() {
    let connection = new WebSocket("http://" + GetApiUrl() + "/create-room");

    ResetGlobalState();
    state.websocketConnection = connection;
    state.isHostPlayer = true;

    document.getElementById("create-room-btn").classList.add("connecting");

    connection.addEventListener("message", ev => HandleConnectionMessage(ev.data));
    connection.addEventListener("open", ev => {
        StartPingLoop();
    });
    connection.onerror = ev => {
        document.getElementById("create-room-btn").classList.remove("connecting");
        Toast("room-creation-failed");
    };
}

// Called once the game can properly start (both players connected)
function OnGameStart() {
    PopulateKeyboard(letter => OnLetterTyped(letter), () => OnEnter(), () => OnBackspace());
    PopulateWordGrids(WORD_LENGTH, MAX_WORD_COUNT, OnSabotageLetter);
    ShowPanel("game-panel");
    state.gameStarted = true;

    state.websocketConnection.onerror = ev => {
        OnDisconnection()
    }

    StartNextTurn();
}

function StartNextTurn() {
    SetGameHint("game-hint-enter-word");
    state.currentTurn += 1;
    state.currentPhase = PHASE_TYPE;
    state.typedWord = "";
    SetLeftGridActive();
}

function StartTypeWaitPhase() {
    state.currentPhase = PHASE_TYPE_WAIT;
    
    state.waitPhaseInterfaceTimeout = setTimeout(() => {  
        SetGameHint("game-hint-wait");
        SetBothGridInactive();
    }, WORD_OK_DELAY);
}

function StartSabotagePhase() {
    state.currentPhase = PHASE_SABOTAGE;
    SetRightGridActive();
    SetSabotageTarget(false, state.currentTurn, true);
    SetGameHint("game-hint-sabotage");
}

function StartSabotageWaitPhase() {
    SetGameHint("game-hint-wait");
    state.currentPhase = PHASE_SABOTAGE_WAIT;
    SetSabotageTarget(false, state.currentTurn, false);
    SetBothGridInactive();
}


function OnLetterTyped(letter) {
    if (state.currentPhase == PHASE_TYPE && state.typedWord.length < WORD_LENGTH) {
        state.typedWord += letter;

        SetLetter(true, state.typedWord.length - 1, state.currentTurn, letter);
    }
}

function OnEnter() {
    if (state.currentPhase == PHASE_TYPE && state.typedWord.length == WORD_LENGTH) {
        if (CheckIfWordIsValid(state.typedWord)) {
            TrySendMessage("word", { word: state.typedWord });
            state.playerWords.push(state.typedWord);
            StartTypeWaitPhase();
        }
        else {
            OnWordRejected();
        }
    }
}

function OnBackspace() {
    if (state.currentPhase == PHASE_TYPE && state.typedWord.length > 0) {
        state.typedWord = state.typedWord.slice(0, -1);
        RemoveLetter(true, state.typedWord.length, state.currentTurn);
    }
}

function OnSabotageLetter(x, y) {
    if (state.currentPhase == PHASE_SABOTAGE && y == state.currentTurn) {
        SetHint(false, x, y, HINT_RED);
        TrySendMessage("sabotage", { id: x });
        StartSabotageWaitPhase();
    }
}

function OnWordRejected() {
    SetGameHint("game-hint-enter-word");
    Toast("toast-invalid-word");
    InvalidAnimation(true, state.currentTurn);
    state.currentPhase = PHASE_TYPE;
    SetLeftGridActive();
}

function TrySendMessage(msgType, msgContent) {
    let msgObject = {
        type: msgType,
        content: msgContent,
    };
    let msgText = JSON.stringify(msgObject);

    if (state.websocketConnection && state.websocketConnection.readyState == WebSocket.OPEN) {
        if (msgType != "ping") { state.lastSentMessage = msgObject; }
        state.websocketConnection.send(msgText);
    }
    else {
        console.log("Couldn't send websocket message");
        state.messagesToSend.push(msgObject);
        OnDisconnection();
    }
}

function StartPingLoop() {
    state.pingLoopHandle = setInterval((() => {
        TrySendMessage("ping", {});
    }), PING_SEND_DELAY);
}

function OnDisconnection() {
    if (state.inReconnectionDelay) { return; } // Already reconnecting

    console.log("Disconnected!");
    Toast("toast-disconnected");
    clearTimeout(state.pingLoopHandle); // Prevent further pings
    state.inReconnectionDelay = true;

    setTimeout(() => { // Wait a little before reconnecting
        state.inReconnectionDelay = false;

        let connection = new WebSocket("http://" + GetApiUrl() + "/reconnect/" + (state.isHostPlayer ? 0 : 1) + "/" + state.roomCode);
        state.websocketConnection = connection;
        
        connection.addEventListener("message", ev => HandleConnectionMessage(ev.data));
        connection.addEventListener("open", ev => {
            Toast("room-reconnected");
            StartPingLoop();

            // Send messages that couldn't be sent during the disconnection
            if (state.lastSentMessage != null) { TrySendMessage(state.lastSentMessage.type, state.lastSentMessage.content); }   
            
            let msgToSendCopy = state.messagesToSend.slice();
            for (let msg of msgToSendCopy) {
                console.log(msg);
                TrySendMessage(msg.type, msg.content);
            }

            state.messagesToSend = [];
        });
        connection.onerror = ev => {
            OnDisconnection();
        };
    }, RECONNECTION_DELAY)
}

function HandleConnectionMessage(msgText) {
    let msg;
    try {
        msg = JSON.parse(msgText);
    }
    catch {
        console.error("Invalid JSON format for websocket message:");
        console.error(msgText);
    }

    if (msg.type == "room-code") {
        state.roomCode = msg.content;
        document.getElementById("room-code").textContent = state.roomCode;

        try {
            navigator.clipboard.writeText(state.roomCode).then();
        }
        catch (e) {
            CustomToast("Couldn't access clipboard. You will have to copy the code by hand. Sorry!");
        }

        ShowPanel("wait-panel");
    }
    else if (msg.type == "other-player-connected") {
        OnGameStart();
    }
    else if (msg.type == "other-player-word") {
        if (state.waitPhaseInterfaceTimeout) {
            clearTimeout(state.waitPhaseInterfaceTimeout)
        }

        SetWord(false, state.currentTurn, msg.content.word);

        if (msg.content.who_wins == "none") {
            state.enemyWords.push(msg.content);
            StartSabotagePhase();
        }
        else if (msg.content.who_wins == "you") {
            SetBothGridActive();
            SetGameHint("hint-win");
            WinAnimation(true, state.currentTurn);
        }
        else if (msg.content.who_wins == "other") {
            SetBothGridActive();
            SetGameHint("hint-loose");
            WinAnimation(false, state.currentTurn);
        }
        else {
            CustomToast("Invalid who_wins value " + msg.content.who_wins);
        }
    }
    else if (msg.type == "word-hints") {
        let hints = msg.content.map(txt => HintTextToId(txt));
        SetHints(true, state.currentTurn, hints);
        UpdateKnownHints(state.typedWord, hints);
        StartNextTurn();
    }
    else if (msg.type == "word-rejected") {
        if (state.waitPhaseInterfaceTimeout) {
            clearTimeout(state.waitPhaseInterfaceTimeout)
        }

        OnWordRejected();
    }
    else {
        console.error("Unknown message type: " + msg.type);
    }
}

function UpdateKnownHints(word, hints) {
    for (let i = 0; i < word.length; i++) {
        if (hints[i] == HINT_YELLOW) {
            if (Object.hasOwn(state.knownHints, word[i])) {
                if (state.knownHints[word[i]] != HINT_GREEN) {
                    state.knownHints[word[i]] = HINT_YELLOW;
                }
            }
            else {
                state.knownHints[word[i]] = HINT_YELLOW;
            }
        }
        else if (hints[i] == HINT_GRAY) {
            if (!Object.hasOwn(state.knownHints, word[i])) {
                state.knownHints[word[i]] = HINT_GRAY;
            }
        }
        else if (hints[i] == HINT_RED) {
            // Nothing
        }
        else {
            state.knownHints[word[i]] = hints[i];
        }
    }

    for (let char in state.knownHints) {
        SetKeyboardHint(char, state.knownHints[char]);
    }
}

// Display the specified hint above the UI
function SetGameHint(hintId) {
    let container = document.getElementById("game-hint");

    for (let element of container.children) {
        if (element.id == hintId) {
            element.classList.remove("hidden")
        }
        else {
            element.classList.add("hidden")
        }
    }
}

function HintTextToId(hintText) {
    if (hintText == "green") { return HINT_GREEN }
    if (hintText == "gray") { return HINT_GRAY }
    if (hintText == "red") { return HINT_RED }
    if (hintText == "yellow") { return HINT_YELLOW }
    if (hintText == "none") { return HINT_NONE }
}

function GetApiUrl() {
    return GetApiUrlWithoutPort() + ":" + API_PORT.toString();
}
function GetApiUrlWithoutPort() {
    let res = window.localStorage.getItem("apiUrl");
    if (res == null) return DEFAULT_API_URL;
    return res;
}
function SetApiUrl(url) {
    window.localStorage.setItem("apiUrl", url)
}

function GetAllWords() {
    fetch("words/english-all.txt").then(res => {
        return res.text()
    }).then(txt => {
        allowedWords = txt.split(";");
    });
}

function CheckIfWordIsValid(word) {
    return allowedWords.includes(word.toLowerCase());
}



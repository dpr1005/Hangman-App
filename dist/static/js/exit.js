const invoke = window.__TAURI__.invoke;

function exitApp() {
    invoke('kill_app')
        .catch(e => {
            alert('An error was encountered while exiting the app');
        });
}

const killButtonYes = document.querySelector("#killButtonYes");
const killButtonNo = document.querySelector("#killButtonNo");

killButtonYes.addEventListener("click", () => {
    exitApp();
});

killButtonNo.addEventListener("click", () => {
    modal.style.display = "none"
});

const newGameButton = document.querySelector("#newGameButton");

newGameButton.addEventListener("click", () => {
    window.location.href = "gameNewLoad.html";
});

const settingsButton = document.querySelector("#SettingsButton");

settingsButton.addEventListener("click", () => {
    window.location.href = "settings.html";
});

let modalBtn = document.getElementById("killButtonModal");
let modal = document.querySelector(".modal");
let closeBtn = document.querySelector(".close-btn");

modalBtn.onclick = function () {
    modal.style.display = "block"
}
closeBtn.onclick = function () {
    modal.style.display = "none"
}
window.onclick = function (e) {
    if (e.target == modal) {
        modal.style.display = "none"
    }
}
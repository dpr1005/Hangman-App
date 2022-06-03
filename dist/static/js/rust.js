//import { message } from "@tauri-apps/api/dialog";
const invoke = window.__TAURI__.invoke;

function exitApp() {
    invoke('kill_app')
    .catch(e => {
        console.log(e);
    });
}

const killButtonSi = document.querySelector("#killButtonSi");
const killButtonNo = document.querySelector("#killButtonNo");

killButtonSi.addEventListener("click", () => {
    exitApp();
});

killButtonNo.addEventListener("click", () => {
    modal.style.display = "none"
});
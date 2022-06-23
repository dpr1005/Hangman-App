const invoke = window.__TAURI__.invoke;

const resetButtonYes = document.querySelector("#resetButtonYes");
const resetButtonNo = document.querySelector("#resetButtonNo");

resetButtonYes.addEventListener("click", () => {
    invoke('reset_database')
        .then(() => {
            alert('The database was reinitialized');
        })
        .catch(e => {
            alert('An error was encountered while resetting the database');
        });
});

resetButtonNo.addEventListener("click", () => {
    modal.style.display = "none"
});

let modalResetDB = document.getElementById("resetDatabaseButton");
let modal = document.querySelector(".modal");
let closeBtn = document.querySelector(".close-btn");

modalResetDB.onclick = function () {
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
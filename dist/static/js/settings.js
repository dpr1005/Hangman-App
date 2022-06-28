const invoke = window.__TAURI__.invoke;
const readFile = window.__TAURI__.fs.readTextFile;
const baseDir = window.__TAURI__.fs.BaseDirectory;
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';

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

const exportDatabaseButton = document.querySelector("#exportDatabaseButton");
const importDatabaseButton = document.querySelector("#importDatabaseButton");

importDatabaseButton.addEventListener("click", () => {
    importDatabase();
});

export function exportDatabase() {
    invoke('export_db')
        .then(() => {
        })
        .catch(e => {
            alert('An error was encountered while exporting the database\n\n' + e);
        });
}

exportDatabaseButton.addEventListener("click", () => {
    
});

const contents = await readTextFile('database/words.csv', { dir: BaseDirectory.App });
console.log(contents);
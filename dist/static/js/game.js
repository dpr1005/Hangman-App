const tauriWindow = window.__TAURI__.window;

let letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
const alphabet = document.getElementById('alphabet');
let passwordBoard;
try {
    passwordBoard = JSON.parse(sessionStorage.getItem('playingWord'));
} catch (error) {
    passwordBoard = [sessionStorage.getItem('playingWord')];
}

const maxTries = 7;
const passwordDiv = document.querySelector('#board');
const imgDiv = document.querySelector('#hangin-dude');
const random = Math.floor(Math.random() * passwordBoard.length);
const password = passwordBoard[random];
let fail = 1;
let countDown;
const start = function () {
    letters.split('').forEach(letter => {
        const html = `<div class="letter">${letter}</div>`;
        alphabet.insertAdjacentHTML('beforeend', html);
    });
    showPassword();
    showHangman(fail);
};
window.onload = start;
const passwordDashed = password.split('').map(letter => {
    if (letter === ' ') return ' ';
    else if (letter === '’') return '’';
    else if (letter === ',') return ',';
    else if (letter === '\'') return '\'';
    else return '_';
});
const showPassword = function () {
    passwordDiv.innerHTML = passwordDashed.join('');
};
const showHangman = function (nr) {
    if (nr >= maxTries)
        nr = maxTries;
    imgDiv.innerHTML = `<img src="static/img/step${nr}.svg" alt="" />`;
};
const checkForLetter = function (e) {
    if (e.target.classList.contains('letter')) {
        if (password.toUpperCase().split('').includes(e.target.textContent)) {
            password
                .toUpperCase()
                .split('')
                .forEach((letter, i, arr) => {
                    if (letter === e.target.textContent) {
                        passwordDashed[i] = letter;
                        showPassword();
                    }
                });
            deactivateLetter(true, e.target);
        } else {
            fail++;
            showHangman(fail);
            deactivateLetter(false, e.target);
        }
        if (fail >= maxTries) {
            finish(false);
        }
        if (password.toUpperCase() === passwordDashed.join('')) {
            finish(true);
        }
    }
};
alphabet.addEventListener('click', checkForLetter);
const deactivateLetter = function (hit, letter, audio) {
    letter.style.border = hit
        ? '1px solid rgb(50, 177, 149)'
        : '1px solid rgba(255, 0, 0, 0.338)';
    letter.style.backgroundColor = hit
        ? 'rgb(50, 177, 149)'
        : 'rgba(255, 0, 0, 0.338)';
    letter.style.color = 'white';
    letter.style.cursor = 'default';
};
const result = document.querySelector('#result');
const finish = function (success) {
    if (success) {
        result.innerHTML = `<h1>NICE WORK!</h1><button id='playAgainButton' class='btn'>PLAY AGAIN</button> <button id='changeWordButton' class='btn'>CHANGE PARAMS</button>`;
        clearInterval(countDown);
        document
            .querySelector('#playAgainButton')
            .addEventListener('click', () => location.reload());
        document
            .querySelector('#changeWordButton')
            .addEventListener('click', () => window.location = 'newGame.html');
    } else {
        result.innerHTML = `<h1>YOU LOST!</h1><button id='tryAgainButton' class='btn'>TRY AGAIN</button><button id='searchWordButton' class='btn'>SEARCH WORD</button>`;
        clearInterval(countDown);
        document
            .querySelector('#tryAgainButton')
            .addEventListener('click', () => window.location = 'newGame.html');
        document
            .querySelector('#searchWordButton')
            .addEventListener('click', () => {
                const webview = new tauriWindow.WebviewWindow('searchWordBrowser', {
                    url: 'https://www.google.com/search?q=' + password,
                });
                webview.once('tauri://created', () => {
                    webview.show();
                });
                webview.once('tauri://error', (error) => {
                    console.error(error);
                });
            });
    }
};
const timer = function () {
    const timer = document.querySelector('#timer');
    let time = new Date(60000);
    const options = {
        minute: '2-digit',
        second: '2-digit',
    };
    const tick = function () {
        time -= 1000;
        timer.textContent = Intl.DateTimeFormat('es-ES', options).format(time);
        if (time == 0) {
            finish(false);
            clearInterval(countDown);
        }
    };
    tick();
    countDown = setInterval(tick, 1000);
    return countDown;
};
timer();
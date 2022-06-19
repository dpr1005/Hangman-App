const invoke = window.__TAURI__.invoke;

function isNumeric(val) {
    return /^-?\d+$/.test(val);
}

function addOptionsToSelect(selectId, options) {
    let select = document.getElementById(selectId);
    let selectLength = select.length;
    for (let i = 0; i < selectLength; i++) {
        select.removeChild(select.lastChild);
    }
    if (!options.includes('all')) {
        options.push('all');
    }
    options.sort((a, b) => ((typeof b === "number") - (typeof a === "number")) || (a > b ? 1 : -1));

    for(let i = 0; i < options.length; i++) {
        let option = document.createElement("option");
        if (!isNumeric(options[i])) {
            option.value = options[i].toLowerCase();
            option.innerHTML = options[i].charAt(0).toUpperCase() + options[i].slice(1);
        } else {
            option.value = options[i];
            option.innerHTML = options[i];
        }
        select.appendChild(option);
    }
}

function findWordTypes() {
    invoke('find_uniques', {
        language: localStorage.getItem('language')
    })
    .then(function (result) {
        addOptionsToSelect('wordTypeSelect', result);
    })
    .catch(e => {
        alert('There was an error while finding the word types');
    });
}

function findWordGroups() {
    invoke('find_groups', {
        language: localStorage.getItem('language')
    })
    .then(function (result) {
        addOptionsToSelect('wordGroupSelect', result);
    })
    .catch(e => {
        alert('There was an error while finding the word groups');
    });
}

function findWordsLength() {
    invoke('find_lengths', {
        wordType: document.getElementById('wordTypeSelect').value,
        group: document.getElementById('wordGroupSelect').value,
        language: localStorage.getItem('language')
    })
    .then(function (result) {
        addOptionsToSelect('wordLengthSelect', result);
        allowStarting();
    })
    .catch(e => {
        alert('There was an error while finding the word lengths');
    });
}

function generateWord() {
    invoke('generate_word', {
        wordType: document.getElementById('wordTypeSelect').value,
        group: document.getElementById('wordGroupSelect').value,
        length: document.getElementById('wordLengthSelect').value,
        language: localStorage.getItem('language')
    })
    .then(function (result) {
        if (result.length > 0) {
            sessionStorage.setItem('playingWord', JSON.stringify(result));
            window.location.href = 'game.html';
        } else {
            alert("There is no words stored in the database that meets the given criteria");
        }
        
    })
    .catch(e => {
        alert('There was an error while generating the word');
    });
}
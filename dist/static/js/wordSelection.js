const invoke = window.__TAURI__.invoke;

function findWordTypes() {
    invoke('get_types', {
        lang: localStorage.getItem('language')
    })
    .then(function (result) {
        addOptionsToSelect('wordTypeSelect', result);
    })
    .catch(e => {
        alert('There was an error while finding the word types');
    });
}

function findWordGroups() {
    invoke('get_groups', {
        lang: localStorage.getItem('language')
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
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
    if (!options.includes('random')) {
        options.push('random');
    }
    options.sort((a, b) => ((typeof b === "number") - (typeof a === "number")) || (a > b ? 1 : -1));
    console.log(options);

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
    invoke('find_uniques')
        .then(function (result) {
            addOptionsToSelect('wordTypeSelect', result);
        })
        .catch(e => {
            console.log(e);
        });
}

function findWordGroups() {
    invoke('find_groups')
        .then(function (result) {
            addOptionsToSelect('wordGroupSelect', result);
        })
        .catch(e => {
            console.log(e);
        });
}

function findWordsLength() {
    invoke('find_lengths', {
        wordType: document.getElementById('wordTypeSelect').value,
        group: document.getElementById('wordGroupSelect').value
    })
    .then(function (result) {
        addOptionsToSelect('wordLengthSelect', result);
    })
    .catch(e => {
        console.log(e);
    });
}

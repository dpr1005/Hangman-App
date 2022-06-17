const invoke = window.__TAURI__.invoke;

function addOptionsToSelect(selectId, options) {
    let select = document.getElementById(selectId);
    if (!options.includes('random')) {
        options.push('random');
    }
    options.sort();

    for(let i = 0; i < options.length; i++) {
        let option = document.createElement("option");
        option.value = options[i].toLowerCase();
        option.innerHTML = options[i].charAt(0).toUpperCase() + options[i].slice(1);
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

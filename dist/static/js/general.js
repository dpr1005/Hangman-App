function isNumeric(val) {
    return /^-?\d+$/.test(val);
}

function addOptionsToSelect(selectId, options, all=true) {
    let select = document.getElementById(selectId);
    let selectLength = select.length;
    for (let i = 0; i < selectLength; i++) {
        select.removeChild(select.lastChild);
    }
    if (!options.includes('all') && all) {
        options.push('all');
    }
    options.sort((a, b) => ((typeof b === "number") - (typeof a === "number")) || (a > b ? 1 : -1));
        
    for (let i = 0; i < options.length; i++) {
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

function getFromDB(func, select, thing, options = {}) {
    invoke(func, options)
        .then((result) => {
            addOptionsToSelect(select, result, thing === 'languages' ? false : true);
        })
        .catch((err) => {
            alert('Error getting ' + thing + ' from the DB');
        });
}

function removeFromDB(func, options = {}) {
    invoke(func, options)
        .then(() => {
            alert('Successfully removed from the Database');
        })
        .catch((err) => {
            alert('Error performing the operation');
        });
}
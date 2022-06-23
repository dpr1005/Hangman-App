const invoke = window.__TAURI__.invoke;

let typesDB = [];
let languagesDB = [];

function addWordToDB(word, lang, type_, group) {

    lang = lang.toUpperCase();
    word = word.toLowerCase();
    type_ = type_.toLowerCase();
    group = group.toLowerCase();

    invoke('add_word', {
        word: word,
        language: lang,
        type: type_,
        group: group
    })
        .then((result) => {
            alert('Word added to DB');
        })
        .catch((err) => {
            alert('Error adding word to DB');
        });
}

function findWords() {
    let form = document.forms['removeWordForm'];
    let lang = form.elements['languagesSelect'].value.toUpperCase();
    let type = form.elements['typesSelect'].value;
    let group = form.elements['groupsSelect'].value;
    console.log(lang, type, group)

    if (lang != "" && type != "" && group != "") {
        invoke('generate_word', {
            wordType: type,
            group: group,
            length: "all",
            language: lang
        })
        .then((result) => {
            console.log(result)
            addOptionsToSelect('wordToRemove', result, false);
            if (result.length > 0) {
                document.getElementById('deleteWordButton').disabled = false;
            }
        })
        .catch((err) => {
            alert('Error getting the matching words from the DB');
            document.getElementById('deleteWordButton').disabled = true;
        })
    } else {
        document.getElementById('deleteWordButton').disabled = true;
    }
}

function deleteWord() {
    let form = document.forms['removeWordForm'];
    let word = form.elements['wordToRemove'].value;
    let lang = form.elements['languagesSelect'].value.toUpperCase();
    let type = form.elements['typesSelect'].value;
    let group = form.elements['groupsSelect'].value;

    console.log(lang, type, group, word);

    invoke('remove_word', {
        word: word,
        lang: lang,
        type: type,
        group: group
    })
        .then((result) => {
            alert('Word deleted from DB');
        })
        .catch((err) => {
            alert('Error deleting word from DB');
        });
    }



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
        .error((err) => {
            alert('Error adding word to DB');
        });
}

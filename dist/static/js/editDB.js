const invoke = window.__TAURI__.invoke;

function addWordToDB(word, lang, type_, group) {
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

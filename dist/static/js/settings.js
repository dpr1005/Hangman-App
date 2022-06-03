const newGameButton = document.querySelector("#newGameButton");

newGameButton.addEventListener("click", () => {
  window.location.href = "gameNewLoad.html";
});

let modalBtn = document.getElementById("killButtonModal");
let modal = document.querySelector(".modal");
let closeBtn = document.querySelector(".close-btn");

modalBtn.onclick = function(){
  modal.style.display = "block"
}
closeBtn.onclick = function(){
  modal.style.display = "none"
}
window.onclick = function(e){
  if(e.target == modal){
    modal.style.display = "none"
  }
}
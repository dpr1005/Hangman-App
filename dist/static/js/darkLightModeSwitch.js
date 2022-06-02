function toggleDarkLight() {
  var body = document.getElementById("body");
  var currentClass = body.className;
  body.className = currentClass == "dark-mode" ? "light-mode" : "dark-mode";
  localStorage.setItem("darkMode", currentClass == "dark-mode" ? "false" : "true");
}

function darkMode() {
  let darkMode = localStorage.getItem("darkMode");
  if(darkMode == null) {
    document.getElementById("body").className = "dark-mode";
    localStorage.setItem("darkMode", "true");
    darkMode = "true";
  }
    
  let body = document.getElementById("body");
  
  if(darkMode == "true") {
    body.className = "dark-mode";
  } else {
    body.className = "light-mode";
  }
}
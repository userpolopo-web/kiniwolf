const toolbar = document.getElementById("toolbar");
const address = document.getElementById("address");
const startAddress = document.getElementById("start-address");

function post(command) {
  if (window.ipc && typeof window.ipc.postMessage === "function") {
    window.ipc.postMessage(JSON.stringify(command));
  }
}

function navigate(value) {
  post({ type: "navigate", value });
}

toolbar.addEventListener("submit", (event) => {
  event.preventDefault();
  navigate(address.value);
});

startAddress.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    navigate(startAddress.value);
  }
});

document.getElementById("back").addEventListener("click", () => post({ type: "back" }));
document.getElementById("forward").addEventListener("click", () => post({ type: "forward" }));
document.getElementById("reload").addEventListener("click", () => post({ type: "reload" }));
document.getElementById("home").addEventListener("click", () => post({ type: "home" }));

window.addEventListener("keydown", (event) => {
  if (event.ctrlKey && event.key.toLowerCase() === "l") {
    event.preventDefault();
    address.focus();
    address.select();
  }
});

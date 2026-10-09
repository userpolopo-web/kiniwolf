const toolbar = document.getElementById("toolbar");
const address = document.getElementById("address");

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

document.getElementById("back").addEventListener("click", () => post({ type: "back" }));
document.getElementById("forward").addEventListener("click", () => post({ type: "forward" }));
document.getElementById("reload").addEventListener("click", () => post({ type: "reload" }));
document.getElementById("home").addEventListener("click", () => post({ type: "home" }));

window.kiniwolfNavigate = (url) => {
  address.value = url;
};

window.addEventListener("keydown", (event) => {
  if (event.ctrlKey && event.key.toLowerCase() === "l") {
    event.preventDefault();
    address.focus();
    address.select();
  }

  if (event.altKey && event.key === "ArrowLeft") {
    event.preventDefault();
    post({ type: "back" });
  }

  if (event.altKey && event.key === "ArrowRight") {
    event.preventDefault();
    post({ type: "forward" });
  }

  if (event.ctrlKey && event.key.toLowerCase() === "r") {
    event.preventDefault();
    post({ type: "reload" });
  }
});

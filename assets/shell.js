const toolbar = document.getElementById("toolbar");
const address = document.getElementById("address");
let currentTab;

window.showError = message => {
  document.getElementById('status').textContent = message;
};
window.renderBrowser = state => {
  currentTab = state.active;
  const list = document.getElementById('tabs');
  list.replaceChildren();
  for (const tab of state.tabs) {
    const item = document.createElement('div');
    item.className = 'tab' + (tab.id === state.active ? ' active' : '') + (tab.suspended ? ' suspended' : '');
    const select = document.createElement('button');
    select.className = 'select';
    select.setAttribute('role', 'tab');
    select.setAttribute('aria-selected', String(tab.id === state.active));
    select.textContent = (tab.suspended ? '◌ ' : '') + tab.title;
    select.title = tab.url;
    select.onclick = () => post({type:'select-tab', id:tab.id});
    const close = document.createElement('button');
    close.className = 'close'; close.textContent = '×'; close.title = 'Fechar aba';
    close.setAttribute('aria-label', 'Fechar ' + tab.title);
    close.onclick = () => post({type:'close-tab', id:tab.id});
    item.append(select, close); list.append(item);
  }
  const active = state.tabs.find(tab => tab.id === state.active);
  if (active && document.activeElement !== address) address.value = active.url;
  document.getElementById('settings').hidden = !state.panel;
  document.getElementById('settings-button').setAttribute('aria-expanded', String(state.panel));
  document.getElementById('memory-saver').checked = state.settings.memory_saver;
  document.getElementById('save-passwords').checked = state.settings.save_passwords;
  document.getElementById('save-passwords').disabled = !state.passwords_supported;
  document.getElementById('save-passwords').closest('label').hidden = !state.passwords_supported;
};

document.getElementById('new-tab').onclick = () => post({type:'new-tab'});
document.getElementById('settings-button').onclick = () => post({type:'panel'});
document.getElementById('memory-saver').onchange = event => post({type:'settings', memory_saver:event.target.checked});
document.getElementById('save-passwords').onchange = event => post({type:'settings', save_passwords:event.target.checked});

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
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 't') { event.preventDefault(); post({type:'new-tab'}); }
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'w') { event.preventDefault(); post({type:'close-tab', id:currentTab}); }
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "l") {
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

  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "r") {
    event.preventDefault();
    post({ type: "reload" });
  }
});
post({type:'ready'});

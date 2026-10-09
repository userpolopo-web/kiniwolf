const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { runInNewContext } = require('node:vm');

function shell() {
  const elements = new Map();
  function element() {
    return {
      value: '', listeners: {}, selected: false,
      addEventListener(name, handler) { this.listeners[name] = handler; },
      setAttribute() {}, replaceChildren() {}, append() {},
      closest() { return this; },
      focus() { document.activeElement = this; this.listeners.focus?.(); },
      blur() { document.activeElement = null; },
      select() { this.selected = true; },
    };
  }
  const document = {
    activeElement: null,
    getElementById(id) {
      if (!elements.has(id)) elements.set(id, element());
      return elements.get(id);
    },
    createElement: element,
  };
  const messages = [];
  const window = { addEventListener() {}, ipc: { postMessage: message => messages.push(JSON.parse(message)) } };
  runInNewContext(readFileSync('assets/shell.js', 'utf8'), { window, document });
  return { window, document, address: document.getElementById('address'), messages };
}

function state(active) {
  return { active, panel: false, passwords_supported: true,
    settings: { memory_saver: false, save_passwords: true, search_engine: 'google' },
    tabs: [ { id: 1, title: 'First', url: 'https://example.com/' },
      { id: 2, title: 'Second', url: 'https://example.org/' } ] };
}

test('focusing the address selects the old URL instead of appending a search', () => {
  const app = shell();
  app.window.renderBrowser(state(1));
  app.address.focus();
  assert.equal(app.address.selected, true);
});

test('changing tabs replaces the address even while the address was focused', () => {
  const app = shell();
  app.window.renderBrowser(state(1));
  app.address.focus();
  app.address.value = 'unfinished search';
  app.window.renderBrowser(state(1));
  assert.equal(app.address.value, 'unfinished search');
  app.window.renderBrowser(state(2));
  assert.equal(app.address.value, 'https://example.org/');
});

test('submitting releases address focus so navigation can display its real URL', () => {
  const app = shell();
  app.address.focus();
  app.address.value = 'rust browser';
  app.document.getElementById('toolbar').listeners.submit({ preventDefault() {} });
  assert.equal(app.messages.at(-1).value, 'rust browser');
  assert.notEqual(app.document.activeElement, app.address);
});

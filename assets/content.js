window.addEventListener('keydown', event => {
  if (event.ctrlKey && ['l', 't', 'w'].includes(event.key.toLowerCase())) {
    event.preventDefault();
    window.ipc.postMessage({l:'focus-address', t:'new-tab', w:'close-tab'}[event.key.toLowerCase()]);
  }
  if (event.altKey && event.key === 'ArrowLeft') { event.preventDefault(); history.back(); }
  if (event.altKey && event.key === 'ArrowRight') { event.preventDefault(); history.forward(); }
  if (event.ctrlKey && event.key.toLowerCase() === 'r') { event.preventDefault(); location.reload(); }
});

// document.getElementById('bar').focus();
const { ipcRenderer } = require('electron')

document.getElementsByTagName('p')[0].innerHTML = 'js works & red if css works';

window.onblur = () => {
  document.getElementsByTagName('p')[0].innerHTML = 'BLURRRR';
  ipcRenderer.send('hide')
}

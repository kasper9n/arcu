const { ipcRenderer } = require('electron')

const barElement = document.getElementById('bar')
const barCopyElement = document.getElementById('bar-copy')
const minitextElement = document.getElementById('minitext')

barElement.focus()

let t0;
let t1;

barElement.addEventListener('input', (e) => {
  t0 = performance.now();
  ipcRenderer.send('search-update', {
    query: barElement.value,
    type: e.inputType,
    data: e.data,
  })
  console.log('input', e)
  barCopyElement.innerText = barElement.value;
})

ipcRenderer.on('results', (e, value) => {
  t1 = performance.now();
  console.log(`results (took ${t1-t0}):`)
  console.log(value)

  minitextElement.innerHTML = " "+value[0].minitext

  t0 = null;
  t1 = null;
})

ipcRenderer.on('after-hide', (e) => {
  barElement.select()
})

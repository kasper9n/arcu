const { ipcRenderer } = require('electron')

function log(...value) {
  ipcRenderer.send('log', value)
};

const barElement = document.getElementById('bar')
const barCopyElement = document.getElementById('bar-copy')
const minitextElement = document.getElementById('minitext')

barElement.focus()

let t0
let t1

barElement.addEventListener('input', (e) => {
  t0 = performance.now()
  ipcRenderer.send('search-update', {
    full_value: barElement.value,
    // type: e.inputType,
    // data: e.data,
  })
  console.log('input', e)
})

ipcRenderer.on('results', (e, value) => {
  t1 = performance.now()
  log('PERFORMANCE: ' + (t1-t0).toFixed(3) + 'ms (input to result)')

  barCopyElement.innerText = barElement.value
  minitextElement.innerHTML = " "+value[0].minitext

  t0 = null
  t1 = null
})

ipcRenderer.on('after-hide', (e) => {
  barElement.select()
})

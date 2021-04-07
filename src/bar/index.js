const { ipcRenderer } = require('electron')

function log(...value) {
  ipcRenderer.send('log', value)
};

const barElement = document.getElementById('bar')
const minitextElement = document.getElementById('minitext')

barElement.focus()

let t0
let t1

barElement.addEventListener('input', (e) => {
  t0 = performance.now()
  ipcRenderer.send('search-update', barElement.innerHTML)
  console.log('input', e)
})

ipcRenderer.on('show', (e, value) => {
  barElement.focus()
})
document.addEventListener('mousedown', (e) => {
  if (e.target !== barElement) {
    e.preventDefault()
  }
  // barElement.focus()
})

ipcRenderer.on('results', (e, value) => {
  t1 = performance.now()
  log('PERFORMANCE: ' + (t1-t0).toFixed(3) + 'ms (input to result)')

  minitextElement.innerHTML = value

  t0 = null
  t1 = null
})

ipcRenderer.on('after-hide', (e) => {
  barElement.select()
})

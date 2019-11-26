const { ipcRenderer } = require('electron')

const barElement = document.getElementById('bar')
barElement.focus()

barElement.addEventListener('input', (e) => {
  ipcRenderer.send('search-update:', {
    query: barElement.value,
    type: e.inputType,
    data: e.data,
  })
})

ipcRenderer.on('results', (e, value) => {
  console.log('results:')
  console.log(value)
})

ipcRenderer.on('after-hide', (e) => {
  barElement.select()
})

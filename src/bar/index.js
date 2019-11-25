const { ipcRenderer } = require('electron')

const barElement = document.getElementById('bar')
barElement.focus()

barElement.addEventListener('input', (e) => {
  console.log('i')
  console.log(e)
  ipcRenderer.send('search-update', {
    query: barElement.value,
    type: e.inputType,
    data: e.data,
  })
})

ipcRenderer.on('after-hide', (e) => {
  barElement.select()
})

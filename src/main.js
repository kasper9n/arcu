const { app, globalShortcut, ipcMain, BrowserWindow, Electron, Menu } = require('electron')
const { performance } = require('perf_hooks')

const addon = require('../native')

const debug = process.env.DEBUG ? true : false

let barWindow = null

app.on('ready', function() {
	barWindow = new BrowserWindow({
		width: 650,
		height: 60,
		show: false,
		resizable: false,
		fullscreenable: false,
    transparent: true,
    frame: false,
    center: true,
    alwaysOnTop: true,
    vibrancy: 'dark',
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
    },
	})
	barWindow.loadFile('src/bar/index.html')

  function hide() {
    app.hide()
    barWindow.webContents.send('after-hide')
  }
  function show() {
    barWindow.show()
    barWindow.webContents.send('show')
  }

  if (!debug) barWindow.on('blur', hide)

  ipcMain.on('search-update', (e, value) => {

    var t0 = performance.now()
    const minitext = addon.query(value)
    var t1 = performance.now()
    console.log('PERFORMANCE: ' + (t1 - t0).toFixed(3) + 'ms (js query)')

    console.log('search-update:', minitext)
    barWindow.webContents.send('results', minitext)
    
  })

	globalShortcut.register('Alt+CommandOrControl+Space', () => {
    // if (debug) barWindow.webContents.openDevTools({ options: { mode: 'detach' } })
    barWindow.isVisible() ? hide() : show()
		console.log(`shortcut pressed`)
	})
})

ipcMain.on('log', (e, value) => {
  console.log.apply(this, value)
})

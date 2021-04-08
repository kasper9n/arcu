const { app, globalShortcut, ipcMain, BrowserWindow, Tray, Menu } = require('electron')
const { performance } = require('perf_hooks')

const isMac = process.platform === 'darwin'

const addon = require('../native')

const debug = process.env.DEBUG ? true : false

let barWindow = null

let tray = null

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
    vibrancy: 'hud',
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
  function toggle() {
    barWindow.isVisible() ? hide() : show()
  }

  tray = new Tray('src/tray-icon/TrayIconTemplate.png')
  if (isMac) {
    tray.on('mouse-down', () => {
      toggle()
    })
  } else {
    tray.on('click', () => {
      toggle()
    })
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

	globalShortcut.register('Alt+Space', () => {
    if (debug) barWindow.webContents.openDevTools({ options: { mode: 'detach' } })
    toggle()
		console.log(`shortcut pressed`)
	})
})

ipcMain.on('log', (e, value) => {
  console.log.apply(this, value)
})

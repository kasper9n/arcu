const { app, globalShortcut, ipcMain, BrowserWindow, Electron, Menu } = require('electron')

const addon = require('../native')

const debug = true

let barWindow = null

app.on('ready', function() {
	barWindow = new BrowserWindow({
		width: 650,
		height: 60,
		show: false,
		resizable: false,
		center: true,
		fullscreenable: false,
    transparent: true,
    frame: false,
    center: true,
    alwaysOnTop: true,
    vibrancy: 'dark',
    webPreferences: {
      nodeIntegration: true,
    },
	})
	barWindow.loadFile('src/bar/index.html')

  function hide() {
    app.hide()
    barWindow.webContents.send('after-hide')
  }
  function show() {
    barWindow.show()
  }

  if (!debug) barWindow.on('blur', hide)

  ipcMain.on('search-update', (e, value) => {
    console.log('search-update:')
    addon.main(value)
  })

	globalShortcut.register('Alt+CommandOrControl+Space', () => {
    barWindow.isVisible() ? hide() : show()
		console.log(`shortcut pressed`)
		if (debug) barWindow.webContents.openDevTools({ options: { mode: 'detach' } })
	})
})

const { app, globalShortcut, ipcMain, BrowserWindow, Tray, Menu, shell } = require('electron')
const { performance } = require('perf_hooks')
const addon = require('../native')

const isMac = process.platform === 'darwin'
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

const template = [
  // { role: 'appMenu' }
  ...(isMac ? [{
    label: app.name,
    submenu: [
      { role: 'about' },
      { type: 'separator' },
      { role: 'services' },
      { type: 'separator' },
      { role: 'hide' },
      { role: 'hideothers' },
      { role: 'unhide' },
      { type: 'separator' },
      { role: 'quit' }
    ]
  }] : []),
  // { role: 'fileMenu' }
  {
    label: 'File',
    submenu: [
      isMac ? { role: 'close' } : { role: 'quit' }
    ]
  },
  // { role: 'editMenu' }
  {
    label: 'Edit',
    submenu: [
      { role: 'undo' },
      { role: 'redo' },
      { type: 'separator' },
      { role: 'cut' },
      { role: 'copy' },
      { role: 'paste' },
      ...(isMac ? [
        { role: 'pasteAndMatchStyle' },
        { role: 'delete' },
        { role: 'selectAll' },
        { type: 'separator' },
        {
          label: 'Speech',
          submenu: [
            { role: 'startSpeaking' },
            { role: 'stopSpeaking' }
          ]
        }
      ] : [
        { role: 'delete' },
        { type: 'separator' },
        { role: 'selectAll' }
      ])
    ]
  },
  // { role: 'viewMenu' }
  {
    label: 'View',
    submenu: [
      { role: 'reload' },
      { role: 'forceReload' },
      { role: 'toggleDevTools' },
    ]
  },
  {
    role: 'help',
    submenu: [
      {
        label: 'Learn More',
        click: async () => {
          await shell.openExternal('https://github.com/probablykasper/arcu')
        }
      }
    ]
  }
]

const menu = Menu.buildFromTemplate(template)
Menu.setApplicationMenu(menu)

const { app, globalShortcut, ipcMain, BrowserWindow } = require('electron')

const addon = require('../native')

console.log(addon.hello())

let barWindow = null;
const debug = false;

app.on('ready', function() {
	const barWindow = new BrowserWindow({
		width: 650,
		height: 60,
		show: false,
		resizable: false,
		center: true,
		fullscreenable: false,
    transparent: true,
    // titleBarStyle: 'hidden',
    frame: false,
    vibrancy: 'dark',
	});
  barWindow.center()
	barWindow.loadFile('src/bar/index.html')


  // ipcMain.on('hide', (event, arg) => {
  //   barWindow.hide()
  // })

  barWindow.on('blur', (event, arg) => {
    barWindow.hide()
  })

	const shortcut = 'Alt+CommandOrControl+Space' // 'CommandOrControl+Space'
	globalShortcut.register(shortcut, () => {
		if (barWindow.isVisible()) {
			barWindow.hide()
      app.hide()
		} else {
			barWindow.show()
		}
		console.log(`${shortcut} is pressed`)
		// barWindow.webContents.openDevTools({ options: { mode: 'detach' } });
	});
});

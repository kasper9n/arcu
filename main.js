const { app, globalShortcut, ipcMain } = require('electron');
const { PanelWindow } = require('electron-panel-window');

let barWindow = null;
const debug = false;

app.on('ready', function() {
	const barWindow = new PanelWindow({
		width: 700,
		height: 100,
		show: false,
		// resizable: false,
		center: true,
		fullscreenable: false,
	}); 
	barWindow.loadFile('src/bar/index.html');

  ipcMain.on('hide', (event, arg) => {
    console.log('hide')
    barWindow.hide()
  })

	const shortcut = 'Alt+CommandOrControl+Space' // 'CommandOrControl+Space'
	globalShortcut.register(shortcut, () => {
		if (barWindow.isVisible()) {
			barWindow.hide();
		} else {
			barWindow.show();
		}
		console.log(`${shortcut} is pressed`);
		// barWindow.webContents.openDevTools({ options: { mode: 'detach' } });
	});
});

const { app, globalShortcut, BrowserWindow } = require('electron');
const { PanelWindow } = require('electron-panel-window');

let barWindow = null;
const debug = false;

app.on('ready', function() {
	const barWindow = new PanelWindow({
		width: 700,
		height: 100,
		show: false,
		resizable: false,
		center: true,
		fullscreenable: false,
		// frameless does not work in PanelWindow
		// frame: false
	}); 
	// setVisibleOnAllWorkspaces crashes the app with PanelWindow
	// barWindow.setVisibleOnAllWorkspaces(true, { visibleOnFullScreen: true });
	// barWindow.loadFile(`./main.html`);
	// barWindow.loadURL(`file://${__dirname}/main.html`);
	barWindow.loadFile('src/bar/index.html');
	if (debug === false) {
		barWindow.on('blur', function(event) {
			console.log('BLUR');
			barWindow.hide();
		});
	}

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

<script>
  import { tauri, window as win, globalShortcut } from '@tauri-apps/api'

  let minitext = ''
  async function onInput(e) {
    try {
      console.log(e.target.innerText)
      console.log(typeof e.target.innerText)
      minitext = await tauri.invoke('query', { value: e.target.innerText })
    } catch (err) {
      console.error(err)
      minitext = err.toString()
    }
  }

  function selectElementContents(el) {
    var range = document.createRange()
    range.selectNodeContents(el)
    var sel = window.getSelection()
    sel.removeAllRanges()
    sel.addRange(range)
  }

  let shown = true
  async function barShortcuts(barElement) {
    globalShortcut.unregisterAll()
    try {
      await globalShortcut.register('Alt+Space', () => {
        shown = !shown
        if (shown) {
          win.appWindow.show()
          barElement.focus()
        } else {
          win.appWindow.hide()
          selectElementContents(barElement)
        }
      })
    } catch (e) {
      console.error(e)
    }
  }
</script>

<style lang="sass">
  :global(body)
    margin: 0
  :global(html)
    background-color: #000000
    border: 1px solid white
    box-sizing: border-box
    height: 100%
    color: white
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif
    overflow: hidden
  :root
    --logo-size:32px
    --window-width:650px
    --bar-padding-top:9px
    --bar-padding-bottom:11px
    --window-base-height:60px
    --input-padding-right:100px
  p
    user-select: none
  main
    height: 60px
    display: flex
    align-items: center
  .logo
    width: var(--logo-size)
    height: var(--logo-size)
    padding: 0px 16px
  .bar
    padding-right: 1px
    font-size: 28px
    border-right: var(--input-padding-right) solid transparent
    white-space: nowrap
    overflow: hidden
    font-weight: 300
    outline: none
  .minitext
    pointer-events: none
    position: relative
    right: var(--input-padding-right)
    width: 0px
    font-size: 14px
    white-space: pre
    opacity: 0.6
    margin-left: 5px
</style>

<main>
  <img class="drag-region logo" alt="logo" src="../logo.svg" />
  <p class="bar" use:barShortcuts contenteditable="true" on:input={onInput} />
  <p class="minitext">{minitext}</p>
</main>

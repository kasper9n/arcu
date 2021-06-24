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
  let barElement

  async function barShortcuts(barElement) {
    globalShortcut.unregisterAll()
    try {
      await globalShortcut.register('Alt+Space', async () => {
        shown = !shown
        if (await win.appWindow.isVisible()) {
          win.appWindow.hide()
          selectElementContents(barElement)
        } else {
          win.appWindow.show()
          setTimeout(() => {
            win.appWindow.setFocus()
          }, 5)
          barElement.focus()
        }
      })
    } catch (e) {
      console.error(e)
    }
  }

  function checkShortcut(e, key, options) {
    const isMac = navigator.userAgent.indexOf('Mac') != -1
    if (e.key.toUpperCase() !== key.toUpperCase()) return false
    if (e.shiftKey !== !!options.shift) return false
    if (e.altKey !== !!options.alt) return false
    if (options.cmdOrControl) {
      if (e.ctrlKey === isMac) return false
      if (e.metaKey === !isMac) return false
    }
    return true
  }

  function keydown(e) {
    if (checkShortcut(e, 'A', { cmdOrControl: true })) {
      console.log('cmd+a')
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

<svelte:window on:keydown={keydown} />
<main>
  <img data-tauri-drag-region class="logo" alt="logo" src="/logo.svg" />
  <p
    class="bar"
    use:barShortcuts
    bind:this={barElement}
    contenteditable="plaintext-only"
    on:input={onInput} />
  <p class="minitext">{minitext}</p>
</main>

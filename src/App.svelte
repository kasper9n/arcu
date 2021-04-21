<script>
  import { appWindow } from '@tauri-apps/api/window'
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { register, unregister, unregisterAll } from '@tauri-apps/api/globalShortcut'

  const originalPosX = appWindow.x
  const originalPosY = appWindow.y
  function resetPos() {
    appWindow.setPosition(originalPosX, originalPosY)
  }

  let minitext = ''
  async function onInput(e) {
    try {
      minitext = await invoke('query', { value: e.target.innerHTML })
    } catch (err) {
      console.error(err)
      minitext = err.toString()
    }
  }

  let barElement
  onMount(async () => {
    let shown = false
    unregisterAll()
    const x = register('Alt+Space', () => {
      shown = !shown
      if (shown) {
        appWindow.show()
        barElement.focus()
      } else {
        appWindow.hide()
        barElement.select()
      }
    })
    console.log(await x)
  })
</script>

<style lang="sass">
  :global(body)
    margin: 0
  :global(html)
    background-color: rgba(0, 0, 0, 1)
    border: 1px solid white
    box-sizing: border-box
    height: 100%
    color: white
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif
  :root
    --logo-size:32px
    --logo-padding-side:16px
    --logo-area-width:calc(var(--logo-size) + 2*var(--logo-padding-side))
    --window-width:650px
    --bar-padding-top:9px
    --bar-padding-bottom:11px
    --window-base-height:60px
    --input-padding-right:100px
  p
    user-select: none
  main
    width: calc(100% + var(--logo-padding-side))
    height: 60px
    display: flex
    align-items: center
  .logo
    width: var(--logo-size)
    height: var(--logo-size)
    padding-left: var(--logo-padding-side)
    padding-right: var(--logo-padding-side)
  .bar
    padding-right: 1px
    font-size: 28px
    border-right: solid var(--input-padding-right) transparent
    white-space: nowrap
    overflow: hidden
    font-weight: 300
    outline: none
  .minitext
    transform: translateX(calc(-1 * var(--input-padding-right)))
    width: 0px
    font-size: 14px
    white-space: pre
    opacity: 0.6
    margin-left: 5px
</style>

<main>
  <img class="drag-region logo" alt="logo" src="../logo.svg" on:dblclick={resetPos} />
  <p class="bar" bind:this={barElement} contenteditable="true" on:input={onInput} />
  <p class="minitext">{minitext}</p>
</main>

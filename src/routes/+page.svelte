<script lang="ts">
	import { tauri, globalShortcut } from '@tauri-apps/api'
	import { onDestroy, onMount } from 'svelte'

	let answer = ''

	let barElement: HTMLParagraphElement
	onMount(async () => {
		globalShortcut.unregister('Alt+Space')
		barElement.focus()
		await globalShortcut.register('Alt+Space', async () => {
			await tauri.invoke('toggle')
			barElement.focus()
		})
	})
	onDestroy(() => {
		globalShortcut.unregister('Alt+Space')
	})

	function checkShortcut(
		e: KeyboardEvent,
		key: string,
		options: { shift?: boolean; alt?: boolean; cmdOrControl?: boolean }
	) {
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

	function keydown(e: KeyboardEvent) {
		if (checkShortcut(e, 'A', { cmdOrControl: true })) {
			console.log('cmd+a')
		}
	}
	function barKeydown(e: KeyboardEvent) {
		if (e.key === 'Tab') {
			e.preventDefault()
		}
	}
</script>

<svelte:window on:keydown={keydown} />

<main on:mousedown|self|preventDefault>
	<div class="bar-container" on:keydown={barKeydown} on:mousedown|self|preventDefault>
		<img
			data-tauri-drag-region
			class="logo"
			alt="logo"
			src="/logo.svg"
			on:mousedown|self|preventDefault
		/>
		<input
			bind:this={barElement}
			type="text"
			class="bar"
			on:input={async (e) => {
				answer = await tauri.invoke('query', { value: e.currentTarget.value })
			}}
			on:blur={() => {
				console.log('BLUR')
			}}
		/>
	</div>
	<p class="answer">{answer}</p>
</main>

<style lang="sass">
	:global(html)
		background-color: #000000
		box-sizing: border-box
		height: 100%
		color: white
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif
		overflow: hidden
		user-select: none
		-webkit-user-select: none
	:global(body)
		margin: 0
		height: 100%
	:root
		--logo-size:32px
		--window-width:650px
		--bar-padding-top:9px
		--bar-padding-bottom:11px
		--window-base-height:60px
		--input-padding-right:100px
	main
		height: 100%
		user-select: none
		-webkit-user-select: none
	.bar-container
		height: 60px
		display: flex
		align-items: center
		pointer-events: all
	.logo
		width: var(--logo-size)
		height: var(--logo-size)
		padding: 0px 16px
	.bar
		color: white
		padding-right: 1px
		font-size: 28px
		border-right: var(--input-padding-right) solid transparent
		white-space: nowrap
		overflow: hidden
		font-weight: 300
		outline: none
		border: none
		background-color: transparent
		pointer-events: all
		width: 100%
	.answer
		margin: 0px
		font-size: 1.5rem
		opacity: 0.6
		width: 100%
		text-align: center
</style>

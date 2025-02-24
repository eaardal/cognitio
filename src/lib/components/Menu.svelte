<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import type { MenuItem, MenuSection as MenuSectionModel, Styling } from '$lib/models';
	import MenuSection from './MenuSection.svelte';
	import { cognitioConfig } from '$lib/stores/config';
	import { cssStringify } from '$lib/helpers/cssUtils';

	const dispatch = createEventDispatcher();

	export let menuSections: MenuSectionModel[] = [];
	export let activeMenuItemId: string | undefined;

	$: menuStyle = stringifyUserMenuStyling($cognitioConfig?.styling?.menu);

	function onMenuItemClick(menuItem: MenuItem): undefined {
		dispatch('menu-item-click', menuItem);
	}

	function onEditCognitioConfigClick() {
		dispatch('edit-cognitio-config-click');
	}

	function propagateMenuItemClick(event: CustomEvent<MenuItem>) {
		onMenuItemClick(event.detail);
	}

	function onEditCheatsheetClick(event: CustomEvent<{ path: string }>) {
		dispatch('edit-cheatsheet-click', event.detail);
	}

	function stringifyUserMenuStyling(styles: Record<string, string> | undefined) {
		if (!styles) {
			return '';
		}

		if (styles?.width && !styles?.minWidth) {
			styles.minWidth = styles.width;
		}

		return cssStringify(styles);
	}

	onMount(() => {
		cognitioConfig.subscribe((config) => {
			menuStyle = stringifyUserMenuStyling(config?.styling?.menu);
		});
	});
</script>

<div class="menu" style={menuStyle}>
	<ul class="menu-list">
		{#if menuSections.length === 0}
			<p>Nothing to show</p>
		{/if}
		{#each menuSections as menuSection}
			<MenuSection
				{menuSection}
				{activeMenuItemId}
				showSectionTitle={menuSections.length > 1}
				on:menu-item-click={propagateMenuItemClick}
				on:edit-click={onEditCheatsheetClick}
			/>
		{/each}
	</ul>
	<div class="bottom">
		<hr />
		<button class="edit-btn" on:click={onEditCognitioConfigClick}>Edit Cognitio Config</button>
	</div>
</div>

<style>
	.menu {
		--width: 225px;
		background-color: var(--foreground);
		min-width: var(--width);
		width: var(--width);
		display: flex;
		flex-direction: column;
		z-index: 1;
		position: sticky;
		top: 0;
		overflow: auto;
		height: 100vh;

		/* Scrollbar */
		--scrollbar-foreground: var(--theme-3);
		--scrollbar-background: var(--theme-2);
		scrollbar-color: var(--scrollbar-foreground) var(--scrollbar-background);
	}

	/* Targetted scrollbar customizations */
	.menu::-webkit-scrollbar {
		width: 10px;
		height: 10px;
	}
	.menu::-webkit-scrollbar-thumb {
		background: var(--scrollbar-foreground);
	}
	.menu::-webkit-scrollbar-track {
		background: var(--scrollbar-background);
	}

	.menu-list {
		list-style: none;
		padding-inline-start: 0;
		padding: 24px;
	}

	.bottom {
		width: 100%;
		height: 50px;
		margin-bottom: 16px;
	}

	.bottom > hr {
		border-color: color-mix(in srgb, var(--foreground) 80%, var(--white));
		color: color-mix(in srgb, var(--foreground) 80%, var(--white));
		display: block;
		height: 1px;
		border: 0;
		border-top: 1px solid color-mix(in srgb, var(--foreground) 80%, var(--white));
		margin: 1em 0;
		padding: 0;
	}

	.edit-btn {
		display: inline-block;
		border: none;
		padding: 2px 4px;
		margin: 0;
		text-decoration: none;
		background: var(--foreground);
		color: color-mix(in srgb, var(--theme-3) 80%, var(--white));
		font-family: 'REM';
		font-size: 0.8rem;
		cursor: pointer;
		text-align: center;
		transition: background 250ms ease-in-out, transform 150ms ease;
		border-radius: 4px;
		margin-left: 24px;
	}

	.edit-btn:hover,
	.edit-btn:focus {
		background: color-mix(in srgb, var(--background) 90%, var(--white));
	}
</style>

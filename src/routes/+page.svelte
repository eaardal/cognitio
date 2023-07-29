<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { parse } from 'yaml';
	import Cheatsheet from '$lib/Cheatsheet.svelte';
	import Menu from '$lib/Menu.svelte';
	import type { Directory, File, Cheatsheet as CheatsheetModel } from '$lib/models';

	let currentCheatsheet: Record<string, string>;

	invoke('greeting')
		.then((value) => {
			console.log('greeting', value);
		})
		.catch((e) => console.error(e));

	function loadCheatsheet(files: File[]) {
		console.log(`loading files`, files);
		invoke('load_cheatsheet', { files })
			.then((value) => {
				console.log('cheatsheet', value);
				currentCheatsheet = value as Record<string, string>;
			})
			.catch((e) => {
				console.error(e);
			});
	}

	function menuItemClicked(event: CustomEvent<Directory>) {
		console.log('clicked', event.detail);
		loadCheatsheet(event.detail.files);
	}
</script>

<div class="root">
	<aside>
		<Menu on:onMenuItemClick={menuItemClicked} />
	</aside>
	<main>
		{#if currentCheatsheet}
			<Cheatsheet cheatsheet={currentCheatsheet} />
		{/if}
	</main>
</div>

<style>
	.root {
		display: flex;
		height: 100%;
	}

	main {
		padding: 24px;
	}
</style>

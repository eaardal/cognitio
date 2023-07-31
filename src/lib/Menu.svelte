<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Directory } from '$lib/models';

	const dispatch = createEventDispatcher();

	let directories: Directory[] = [];

	invoke('list_cheatsheet_directories').then((value) => {
		directories = value as Directory[];
	});

	function onMenuItemClick(directory: Directory): undefined {
		dispatch('onMenuItemClick', directory);
	}
</script>

<div class="menu">
	<ul class="menu-list">
		{#each directories as directory}
			<li class="menu-item">
				<a href="#" on:click={onMenuItemClick(directory)}>{directory.name}</a>
			</li>
		{/each}
	</ul>
</div>

<style>
	.menu {
		background-color: var(--foreground);
		min-width: 200px;
		width: 200px;
		height: 100%;
	}

	.menu-list {
		padding: 16px;
	}

	.menu-item {
		font-size: 1em;
		color: var(--white);
	}
</style>

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

<nav class="menu">
	<ul>
		{#each directories as directory}
			<li><a href="#" on:click={onMenuItemClick(directory)}>{directory.name}</a></li>
		{/each}
	</ul>
</nav>

<style>
	.menu {
		background-color: lightblue;
		padding: 24px;
		height: 100%;
	}
</style>

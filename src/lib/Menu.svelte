<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Directory } from '$lib/models';

	const dispatch = createEventDispatcher();

	export let directories: Directory[] = [];
	export let activeDirectory: Directory | undefined;

	function onMenuItemClick(directory: Directory): undefined {
		dispatch('onMenuItemClick', directory);
	}
</script>

<div class="menu">
	<ul class="menu-list">
		{#each directories as directory}
			<li class="menu-item" class:active={activeDirectory?.name === directory.name}>
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
		list-style: none;
		padding-inline-start: 0;
		padding: 24px;
	}

	.menu-item {
		font-size: 1em;
		color: var(--white);
	}

	.active > a {
		font-weight: 600;
		color: var(--white);
	}
</style>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Directory } from '$lib/models';
	import Tooltip from './Tooltip.svelte';

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
			<div class="menu-section">
				{#if directory.sub_directories.length > 0 && directory.files.length === 0}
					<Tooltip text={directory.path}>
						<li class="menu-item root-directory">
							<div>{directory.name}</div>
						</li>
					</Tooltip>
					{#each directory.sub_directories as subDirectory}
						<li class="menu-item" class:active={activeDirectory?.path === subDirectory.path}>
							<a href="#" on:click={onMenuItemClick(subDirectory)}>{subDirectory.name}</a>
						</li>
					{/each}
				{/if}
			</div>
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

	.menu-section {
		margin-bottom: 16px;
	}

	.root-directory div {
		color: color-mix(in srgb, var(--foreground) 30%, var(--white));
		text-transform: uppercase;
		font-size: 0.8em;
		letter-spacing: 1.3px;
	}
</style>

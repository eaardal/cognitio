<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Directory } from '$lib/models';
	import Tooltip from '$lib/components/Tooltip.svelte';

	const dispatch = createEventDispatcher();

	export let directories: Directory[] = [];
	export let activeDirectory: Directory | undefined;

	function onMenuItemClick(directory: Directory): undefined {
		dispatch('menu-item-click', directory);
	}

	function onEditCognitioConfigClick() {
		dispatch('edit-cognitio-config-click');
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
							<a href="#." on:click={onMenuItemClick(subDirectory)}>{subDirectory.name}</a>
						</li>
					{/each}
				{/if}
			</div>
		{/each}
	</ul>
	<div class="bottom">
		<hr />
		<button class="edit-btn" on:click={onEditCognitioConfigClick}>Edit Cognitio Config</button>
	</div>
</div>

<style>
	.menu {
		background-color: var(--foreground);
		min-width: 200px;
		width: 200px;
		display: flex;
		flex-direction: column;
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

	.bottom {
		width: 100%;
		height: 50px;
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

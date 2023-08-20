<script lang="ts">
	import { afterUpdate, createEventDispatcher } from 'svelte';
	import '$lib/components/Cheatsheet.css';
	import { addEventHandlersToCopyButtons } from '$lib/helpers/copyButton';

	export let cheatsheet: Record<string, string>;
	export let name: string;
	export let path: string;

	const dispatch = createEventDispatcher();

	function onEditDirectoryClick() {
		dispatch('edit-directory', { path, name });
	}

	function onEditFileClick(fileNameWithoutExt: string) {
		const filePath = `${path}/${fileNameWithoutExt}.md`;
		dispatch('edit-file', { path: filePath, name: fileNameWithoutExt });
	}

	afterUpdate(() => {
		addEventHandlersToCopyButtons();
	});
</script>

<div class="cheatsheet">
	<div class="header">
		<h2>{name}</h2>
		<button class="edit-btn" on:click={onEditDirectoryClick}>Edit</button>
	</div>
	<div class="section-shortcuts">
		{#each Object.keys(cheatsheet) as sectionName}
			<a href={`#${sectionName}`}>{sectionName}</a>
		{/each}
	</div>
	<div class="content">
		{#each Object.keys(cheatsheet) as sectionName, index}
			<div class="section-root">
				<!-- svelte-ignore a11y-missing-content -->
				<a id={sectionName} />

				<h3 class="section-title">
					{sectionName}<button
						class="go-to-top-btn"
						on:click={() => {
							window.scrollTo({ top: 0, behavior: 'smooth' });
						}}>Top</button
					>
					<button class="edit-btn" on:click={() => onEditFileClick(sectionName)}>Edit</button>
				</h3>

				<!-- eslint-disable svelte/no-at-html-tags -->
				<div class="cheatsheet-html-root">{@html cheatsheet[sectionName]}</div>
				<!-- eslint-enable -->

				{#if Object.keys(cheatsheet).length > 1 && index !== Object.keys(cheatsheet).length - 1}
					<hr class="section-end" />
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.cheatsheet {
		display: flex;
		flex-direction: column;
	}

	.cheatsheet-html-root {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}

	.header {
		--padding-sides: 24px;
		background-color: color-mix(in srgb, var(--background) 90%, var(--white));
		width: calc(100%-var(--padding-sides));
		height: 50px;
		padding-left: var(--padding-sides);
		padding-right: var(--padding-sides);
		display: flex;
		align-items: center;
		z-index: 1;
		box-shadow: 0 0 4px 0 rgba(0, 0, 0, 0.3);
	}

	.header > h2 {
		font-size: 1.2em;
		margin: 0;
		padding: 0;
		color: var(--theme-3);
	}

	.section-title {
		display: flex;
		align-items: center;
	}

	.go-to-top-btn {
		display: inline-block;
		border: none;
		padding: 2px 4px;
		margin: 0;
		text-decoration: none;
		background: var(--foreground);
		color: var(--accent);
		font-family: 'REM';
		font-size: 0.8rem;
		cursor: pointer;
		text-align: center;
		transition: background 250ms ease-in-out, transform 150ms ease;
		border-radius: 4px;
		margin-left: 16px;
	}

	.go-to-top-btn:hover,
	.go-to-top-btn:focus {
		background: var(--foreground-lighter);
	}

	.section-shortcuts {
		--padding-sides: 24px;
		background-color: color-mix(in srgb, var(--background) 95%, var(--white));
		width: calc(100%-var(--padding-sides));
		max-height: 100px;
		padding-left: var(--padding-sides);
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		padding-top: 8px;
		padding-bottom: 8px;
		box-shadow: 0 0 4px 0 rgba(0, 0, 0, 0.3);
	}

	.section-shortcuts > a {
		margin-right: 16px;
	}

	.content {
		margin-left: 24px;
		margin-right: 24px;
	}

	.section-root {
		margin-bottom: 40px;
	}

	.section-end {
		margin-top: 32px;
		border-color: color-mix(in srgb, var(--background) 80%, var(--white));
		color: color-mix(in srgb, var(--background) 80%, var(--white));
		display: block;
		height: 1px;
		border: 0;
		border-top: 1px solid color-mix(in srgb, var(--background) 80%, var(--white));
		padding: 0;
	}

	.edit-btn {
		display: inline-block;
		border: none;
		padding: 2px 4px;
		margin: 0;
		text-decoration: none;
		background: var(--foreground);
		color: var(--accent);
		font-family: sans-serif;
		font-size: 0.8rem;
		cursor: pointer;
		text-align: center;
		transition: background 250ms ease-in-out, transform 150ms ease;
		border-radius: 4px;
		margin-left: 16px;
	}

	.edit-btn:hover,
	.edit-btn:focus {
		background: var(--foreground-lighter);
	}
</style>

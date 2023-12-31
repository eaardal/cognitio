<script lang="ts">
	import { afterUpdate, createEventDispatcher } from 'svelte';
	import '$lib/components/Cheatsheet.css';
	import { addEventHandlersToCopyButtons } from '$lib/helpers/copyButton';
	import SectionHeader from './SectionHeader.svelte';
	import MarkdownContent from './MarkdownContent.svelte';
	import { pathToHash } from '$lib/helpers/hashUtils';

	export let cheatsheet: Record<string, string>;
	export let name: string;
	export let path: string;

	const dispatch = createEventDispatcher();

	function onEditDirectoryClick() {
		dispatch('edit-directory', { path, name });
	}

	function onEditFileClick(event: CustomEvent<{ name: string }>) {
		const filePath = `${path}/${event.detail.name}.md`;
		dispatch('edit-file', { path: filePath, name: event.detail.name });
	}

	function appendSectionNameToDirectoryPath(sectionName: string) {
		return `${path}/${sectionName}.md`;
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
			<a href={`#section_${pathToHash(appendSectionNameToDirectoryPath(sectionName))}`}
				>{sectionName}</a
			>
		{/each}
	</div>
	<div class="content">
		{#each Object.keys(cheatsheet) as sectionName, index}
			<div class="section-root">
				<SectionHeader
					id={appendSectionNameToDirectoryPath(sectionName)}
					name={sectionName}
					on:edit-click={onEditFileClick}
				/>
				<MarkdownContent markdown={cheatsheet[sectionName]} />

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

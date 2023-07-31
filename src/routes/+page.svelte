<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import dompurify from 'dompurify';
	import { marked } from '$lib/markdown';
	import Cheatsheet from '$lib/Cheatsheet.svelte';
	import Menu from '$lib/Menu.svelte';
	import type { Directory, File } from '$lib/models';

	let currentCheatsheet: Record<string, string>;

	function loadCheatsheet(files: File[]) {
		invoke('load_cheatsheet', { files })
			.then((value) => {
				const rawCheatsheet = value as Record<string, string>;

				const cheatsheet = Object.keys(rawCheatsheet).reduce((all, key) => {
					const markdown = rawCheatsheet[key] as string;
					const sanitizedMarkdown = dompurify.sanitize(markdown);
					const html = marked.parse(sanitizedMarkdown);
					return html ? { ...all, [key]: html } : all;
				}, {} as Record<string, string>);

				currentCheatsheet = cheatsheet;
			})
			.catch((e) => {
				console.error(e);
			});
	}

	function menuItemClicked(event: CustomEvent<Directory>) {
		loadCheatsheet(event.detail.files);
	}
</script>

<div class="page-root">
	<Menu on:onMenuItemClick={menuItemClicked} />

	<div class="page-content">
		{#if currentCheatsheet}
			<Cheatsheet cheatsheet={currentCheatsheet} />
		{/if}
	</div>
</div>

<style>
	.page-root {
		height: 100%;
		display: flex;
		flex-direction: row;
	}

	.page-content {
		padding: 24px;
		padding-top: 0;
	}
</style>

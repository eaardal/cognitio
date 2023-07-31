<script lang="ts">
	import 'highlight.js/styles/github-dark.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import { marked } from 'marked';
	import { markedHighlight } from 'marked-highlight';
	import dompurify from 'dompurify';
	import hljs from 'highlight.js';
	import Cheatsheet from '$lib/Cheatsheet.svelte';
	import Menu from '$lib/Menu.svelte';
	import type { Directory, File } from '$lib/models';

	hljs.highlightAll();

	marked.use(
		{
			mangle: false,
			headerIds: false
		},
		markedHighlight({
			highlight(code, lang) {
				const language = hljs.getLanguage(lang) ? lang : 'plaintext';
				return hljs.highlight(code, { language }).value;
			}
		})
	);

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
	}
</style>

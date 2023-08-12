<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { Event, UnlistenFn } from '@tauri-apps/api/event';
	import dompurify from 'dompurify';
	import { marked } from '$lib/markdown';
	import Cheatsheet from '$lib/Cheatsheet.svelte';
	import Menu from '$lib/Menu.svelte';
	import type { Directory, File, FileChangedPayload } from '$lib/models';
	import { onMount, onDestroy } from 'svelte';

	let cheatsheetDirectories: Directory[];
	let currentDirectory: Directory | undefined;
	let currentCheatsheet: Record<string, string> | undefined;
	let subscriptions: UnlistenFn[] = [];

	function loadCheatsheet(files: File[]) {
		invoke('load_cheatsheet', { files })
			.then((value) => {
				console.log('load_cheatsheet', value);
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

	function loadCheatsheetDirectories() {
		invoke('list_cheatsheet_directories')
			.then((value) => {
				console.log('load_cheatsheet_directories', value);
				cheatsheetDirectories = value as Directory[];
				return value as Directory[];
			})
			.then((directories) => {
				if (directories && directories.length > 0) {
					currentDirectory = directories[0];
					loadCheatsheet(currentDirectory.files);
				}
			});
	}

	function menuItemClicked(event: CustomEvent<Directory>) {
		currentDirectory = event.detail;
		loadCheatsheet(event.detail.files);
	}

	onMount(async () => {
		loadCheatsheetDirectories();

		const unlisten = await listen('file-changed', (event: Event<FileChangedPayload>) => {
			if (event.event === 'file-changed') {
				loadCheatsheetDirectories();
			}
		});
		subscriptions.push(unlisten);
	});

	onDestroy(async () => {
		for (const sub of subscriptions) {
			sub();
		}
	});
</script>

<div class="page-root">
	<Menu
		directories={cheatsheetDirectories}
		activeDirectory={currentDirectory}
		on:onMenuItemClick={menuItemClicked}
	/>

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

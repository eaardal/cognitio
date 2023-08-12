<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { Event, UnlistenFn } from '@tauri-apps/api/event';
	import dompurify from 'dompurify';
	import { marked } from '$lib/helpers/marked';
	import Cheatsheet from '$lib/components/Cheatsheet.svelte';
	import Menu from '$lib/components/Menu.svelte';
	import type { Directory, File, FileChangedPayload } from '$lib/models';

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

	function findFirstDirectoryWithCheatsheets(directories: Directory[]): Directory | undefined {
		let directoryToLoad: Directory | undefined;

		for (const directory of directories) {
			if (directory.files.length > 0) {
				directoryToLoad = directory;
				break;
			}

			if (directory.sub_directories.length > 0) {
				directoryToLoad = findFirstDirectoryWithCheatsheets(directory.sub_directories);
				if (directoryToLoad) {
					break;
				}
			}
		}

		return directoryToLoad;
	}

	function loadCheatsheetDirectories() {
		invoke('list_cheatsheet_directories')
			.then((value) => {
				cheatsheetDirectories = value as Directory[];
				return value as Directory[];
			})
			.then((directories) => {
				const directoryToLoad = findFirstDirectoryWithCheatsheets(directories);
				if (directoryToLoad && directoryToLoad?.files.length > 0) {
					currentDirectory = directoryToLoad;
					loadCheatsheet(directoryToLoad.files);
				}
			});
	}

	function menuItemClicked(event: CustomEvent<Directory>) {
		currentDirectory = event.detail;
		loadCheatsheet(event.detail.files);
	}

	function editCheatsheet(event: CustomEvent<{ path: string; name: string }>) {
		invoke('edit_cheatsheet', { path: event.detail.path }).catch((e) => {
			console.error(e);
		});
	}

	function editCognitioConfig() {
		invoke('edit_cognitio_config').catch((e) => {
			console.error(e);
		});
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

	console.log('currentCheatsheet', currentCheatsheet);
</script>

<div class="page-root">
	<Menu
		directories={cheatsheetDirectories}
		activeDirectory={currentDirectory}
		on:menu-item-click={menuItemClicked}
		on:edit-cognitio-config-click={editCognitioConfig}
	/>

	<div class="page-content">
		{#if !currentCheatsheet || !currentDirectory}
			<h2>No cheatsheet selected</h2>
			<p>Select a cheatsheet in the left-side menu.</p>
		{:else}
			<Cheatsheet
				path={currentDirectory.path}
				name={currentDirectory?.name}
				cheatsheet={currentCheatsheet}
				on:edit-cheatsheet={editCheatsheet}
			/>
		{/if}
	</div>
</div>

<style>
	.page-root {
		height: 100%;
		min-height: 100vh;
		display: flex;
		flex-direction: row;
	}

	.page-content {
		padding-top: 0;
	}
</style>

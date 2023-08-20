<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { Event, UnlistenFn } from '@tauri-apps/api/event';
	import dompurify from 'dompurify';
	import orderBy from 'lodash/orderBy';
	import { marked } from '$lib/helpers/marked';
	import Cheatsheet from '$lib/components/Cheatsheet.svelte';
	import Menu from '$lib/components/Menu.svelte';
	import type { Directory, File, FileChangedPayload } from '$lib/models';
	import {
		invokeEditCognitioConfigCommand,
		invokeEditDirectoryCommand,
		invokeEditFileCommand,
		invokeLoadCheatsheetCommand,
		invokeLoadCheatsheetDirectoriesCommand,
		listenForFileChangedEvents
	} from '$lib/helpers/tauri';

	let cheatsheetDirectories: Directory[];
	$: cheatsheetDirectories = [];

	let currentDirectory: Directory | undefined;
	$: currentDirectory = undefined;

	let currentCheatsheet: Record<string, string> | undefined;
	$: currentCheatsheet = undefined;

	let subscriptions: UnlistenFn[] = [];

	async function loadCheatsheet(files: File[]) {
		try {
			const rawCheatsheet = await invokeLoadCheatsheetCommand(files);

			const cheatsheet = Object.keys(rawCheatsheet).reduce((all, key) => {
				const markdown = rawCheatsheet[key] as string;
				const sanitizedMarkdown = dompurify.sanitize(markdown);
				const html = marked.parse(sanitizedMarkdown);
				return html ? { ...all, [key]: html } : all;
			}, {} as Record<string, string>);

			currentCheatsheet = cheatsheet;
		} catch (error) {
			console.error('Failed to load cheatsheet', error);
		}
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

	function orderAllDirectoriesByName(directories: Directory[]): Directory[] {
		const orderedDirectories = orderBy(directories, ['name'], ['asc']);
		for (const dir of orderedDirectories) {
			dir.sub_directories = orderBy(dir.sub_directories, ['name'], ['asc']);
		}
		return orderedDirectories;
	}

	async function loadCheatsheetDirectories() {
		try {
			const directories = await invokeLoadCheatsheetDirectoriesCommand();
			cheatsheetDirectories = orderAllDirectoriesByName(directories);
			const directoryToLoad = findFirstDirectoryWithCheatsheets(cheatsheetDirectories);
			if (directoryToLoad && directoryToLoad?.files.length > 0) {
				currentDirectory = directoryToLoad;
				await loadCheatsheet(currentDirectory.files);
			}
		} catch (error) {
			console.error('Failed to load cheatsheet directories', error);
		}
	}

	async function menuItemClicked(event: CustomEvent<Directory>) {
		currentDirectory = event.detail;
		await loadCheatsheet(event.detail.files);
	}

	function editDirectory(event: CustomEvent<{ path: string; name: string }>) {
		invokeEditDirectoryCommand(event.detail.path).catch((e) => {
			console.error('Failed to invoke edit directory command', e);
		});
	}

	function editFile(event: CustomEvent<{ path: string; name: string }>) {
		invokeEditFileCommand(event.detail.path).catch((e) => {
			console.error('Failed to invoke edit file command', e);
		});
	}

	function editCognitioConfig() {
		invokeEditCognitioConfigCommand().catch((e) => {
			console.error('Failed to invoke edit cognitio config command', e);
		});
	}

	onMount(() => {
		async function initialize() {
			await loadCheatsheetDirectories();

			const unlisten = await listenForFileChangedEvents(
				async (event: Event<FileChangedPayload>) => {
					if (event.event === 'file-changed') {
						await loadCheatsheetDirectories();
					}
				}
			);
			subscriptions.push(unlisten);
		}

		initialize();

		return () => {
			for (const unsub of subscriptions) {
				unsub();
			}
		};
	});
</script>

{#key currentCheatsheet}
	<div class="page-root">
		<Menu
			directories={cheatsheetDirectories}
			activeDirectory={currentDirectory}
			on:menu-item-click={menuItemClicked}
			on:edit-cognitio-config-click={editCognitioConfig}
		/>

		{#if !cheatsheetDirectories}
			<div class="page-content">
				<div class="error-message">
					<h2>Nothing to show</h2>
					<p>There appears to be no cheatsheet directories listed in cognitio.yaml.</p>
					<p>
						<button class="edit-btn" on:click={editCognitioConfig}
							>Edit the Cognitio config file</button
						> to add a directory and get started.
					</p>
				</div>
			</div>
		{:else}
			<div class="page-content">
				{#if !currentCheatsheet || !currentDirectory}
					<div class="error-message">
						<h2>No cheatsheet selected</h2>
						<p>Select a cheatsheet in the left-side menu.</p>
					</div>
				{:else}
					<Cheatsheet
						path={currentDirectory.path}
						name={currentDirectory?.name}
						cheatsheet={currentCheatsheet}
						on:edit-directory={editDirectory}
						on:edit-file={editFile}
					/>
				{/if}
			</div>
		{/if}
	</div>
{/key}

<style>
	.page-root {
		height: 100%;
		min-height: 100vh;
		display: flex;
		flex-direction: row;
	}

	.page-content {
		padding-top: 0;
		width: 100%;
	}

	.error-message {
		padding-left: 24px;
	}

	.edit-btn {
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
	}
</style>

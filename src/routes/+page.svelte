<script lang="ts">
	import { onMount } from 'svelte';
	import type { Event, UnlistenFn } from '@tauri-apps/api/event';
	import orderBy from 'lodash/orderBy';
	import { cheatsheetToMarkdown } from '$lib/helpers/marked';
	import Cheatsheet from '$lib/components/Cheatsheet.svelte';
	import Menu from '$lib/components/Menu.svelte';
	import type { Directory, File, FileChangedPayload, MenuItem, MenuSection } from '$lib/models';
	import {
		invokeEditCognitioConfigCommand,
		invokeEditDirectoryCommand,
		invokeEditFileCommand,
		invokeLoadCheatsheetCommand,
		invokeLoadCheatsheetDirectoriesCommand,
		invokeLoadCheatsheetSectionCommand,
		listenForFileChangedEvents
	} from '$lib/helpers/tauri';
	import { mapDirectoriesToMenuSections } from '$lib/models/mapping';

	let cheatsheetDirectories: Directory[];
	$: cheatsheetDirectories = [];

	let currentDirectory: Directory | undefined;
	$: currentDirectory = undefined;

	let currentCheatsheet: Record<string, string> | undefined;
	$: currentCheatsheet = undefined;

	let menuSections: MenuSection[];
	$: menuSections = [];

	let activeMenuItem: MenuItem | undefined;
	$: activeMenuItem = undefined;

	let subscriptions: UnlistenFn[] = [];

	async function loadCheatsheet(files: File[]) {
		try {
			const rawCheatsheet = await invokeLoadCheatsheetCommand(files);
			currentCheatsheet = cheatsheetToMarkdown(rawCheatsheet);
		} catch (error) {
			console.error('Failed to load cheatsheet', error);
		}
	}

	async function loadCheatsheetSection(path: string) {
		try {
			const rawCheatsheet = await invokeLoadCheatsheetSectionCommand(path);
			const updatedCheatsheet = cheatsheetToMarkdown(rawCheatsheet);

			if (!currentCheatsheet) {
				currentCheatsheet = updatedCheatsheet;
				return;
			}

			Object.keys(rawCheatsheet).forEach((key) => {
				currentCheatsheet![key] = updatedCheatsheet[key];
			});
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
			menuSections = mapDirectoriesToMenuSections(cheatsheetDirectories);
			const directoryToLoad = findFirstDirectoryWithCheatsheets(cheatsheetDirectories);
			if (directoryToLoad && directoryToLoad?.files.length > 0) {
				currentDirectory = directoryToLoad;
				await loadCheatsheet(currentDirectory.files);
			}
		} catch (error) {
			console.error('Failed to load cheatsheet directories', error);
		}
	}

	function findDirectoryWithPath(path: string): Directory {
		let directory: Directory | undefined;

		for (const dir of cheatsheetDirectories) {
			if (dir.path === path) {
				directory = dir;
				break;
			}

			if (dir.sub_directories.length > 0) {
				for (const subDirectory of dir.sub_directories) {
					if (subDirectory.path === path) {
						directory = subDirectory;
						break;
					}
				}
			}
		}

		return directory!;
	}

	async function menuItemClicked(event: CustomEvent<MenuItem>) {
		const menuItemAsDirectory = findDirectoryWithPath(event.detail.id);
		currentDirectory = menuItemAsDirectory;
		activeMenuItem = event.detail;
		await loadCheatsheet(menuItemAsDirectory!.files);
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

			const unlisten = await listenForFileChangedEvents((event: Event<FileChangedPayload>) => {
				void loadCheatsheetSection(event.payload.path);
			});
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
			{menuSections}
			active={activeMenuItem}
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

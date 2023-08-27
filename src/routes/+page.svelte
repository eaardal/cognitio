<script lang="ts">
	import { onMount } from 'svelte';
	import type { Event, UnlistenFn } from '@tauri-apps/api/event';
	import { cheatsheetToMarkdown } from '$lib/helpers/marked';
	import Cheatsheet from '$lib/components/Cheatsheet.svelte';
	import Menu from '$lib/components/Menu.svelte';
	import type {
		CognitioConfigChangedPayload,
		Directory,
		File,
		FileChangedPayload,
		MenuItem,
		MenuSection
	} from '$lib/models';
	import {
		invokeEditCognitioConfigCommand,
		invokeEditDirectoryCommand,
		invokeEditFileCommand,
		invokeLoadCheatsheetCommand,
		invokeLoadCheatsheetDirectoriesCommand,
		invokeLoadCheatsheetSectionCommand,
		invokeLoadCognitioConfigCommand,
		listenForCognitioConfigChangedEvents,
		listenForFileChangedEvents
	} from '$lib/helpers/tauri';
	import { mapDirectoriesToMenuSections, orderMenuSections } from '$lib/models/mapping';
	import { cognitioConfig } from '$lib/stores/config';
	import {
		findDirectoryWithPath,
		findFirstDirectoryWithCheatsheets,
		orderAllDirectoriesByName
	} from '$lib/helpers/directoryUtils';
	import { orderBy } from 'lodash';

	let cheatsheetDirectories: Directory[];
	$: cheatsheetDirectories = [];

	let currentDirectory: Directory | undefined;
	$: currentDirectory = undefined;

	let currentCheatsheet: Record<string, string> | undefined;
	$: currentCheatsheet = undefined;

	let menuSections: MenuSection[];
	$: menuSections = [];

	let activeMenuItemId: string | undefined;
	$: activeMenuItemId = undefined;

	let subscriptions: UnlistenFn[] = [];

	async function loadCognitioConfig() {
		const config = await invokeLoadCognitioConfigCommand();
		cognitioConfig.set(config);
	}

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

	async function loadCheatsheetDirectories() {
		try {
			const directories = await invokeLoadCheatsheetDirectoriesCommand();
			cheatsheetDirectories = orderAllDirectoriesByName(directories);
			const menu = mapDirectoriesToMenuSections(cheatsheetDirectories);
			menuSections = orderMenuSections(menu);
			const directoryToLoad = findFirstDirectoryWithCheatsheets(cheatsheetDirectories);
			if (directoryToLoad && directoryToLoad?.files.length > 0) {
				currentDirectory = directoryToLoad;
				activeMenuItemId = directoryToLoad.path;
				await loadCheatsheet(currentDirectory.files);
			}
		} catch (error) {
			console.error('Failed to load cheatsheet directories', error);
		}
	}

	async function menuItemClicked(event: CustomEvent<MenuItem>) {
		const menuItemAsDirectory = findDirectoryWithPath(cheatsheetDirectories, event.detail.id);
		if (menuItemAsDirectory) {
			currentDirectory = menuItemAsDirectory;
			activeMenuItemId = event.detail.id;
			await loadCheatsheet(menuItemAsDirectory!.files);
			window.scrollTo({
				top: 0,
				behavior: 'smooth'
			});
		}
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
			await loadCognitioConfig();

			const unlistenFileChanged = await listenForFileChangedEvents(
				(event: Event<FileChangedPayload>) => {
					void loadCheatsheetSection(event.payload.path);
				}
			);
			subscriptions.push(unlistenFileChanged);

			const unlistenConfigChanged = await listenForCognitioConfigChangedEvents(
				(event: Event<CognitioConfigChangedPayload>) => {
					cognitioConfig.update(() => event.payload.config);
					void loadCheatsheetDirectories();
				}
			);
			subscriptions.push(unlistenConfigChanged);
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
			{activeMenuItemId}
			on:menu-item-click={menuItemClicked}
			on:edit-cognitio-config-click={editCognitioConfig}
		/>

		{#if typeof cheatsheetDirectories === 'undefined' || menuSections?.length === 0}
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

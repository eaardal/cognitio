import type {
	CognitioConfig,
	CognitioConfigChangedPayload,
	Directory,
	File,
	FileChangedPayload
} from '$lib/models';
import { listen, type UnlistenFn, type Event } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export function invokeEditDirectoryCommand(path: string): Promise<string> {
	return invoke('edit_directory', { path });
}

export function invokeEditFileCommand(path: string): Promise<string> {
	return invoke('edit_file', { path });
}

export function invokeEditCognitioConfigCommand(): Promise<string> {
	return invoke('edit_cognitio_config');
}

export function invokeLoadCheatsheetCommand(files: File[]): Promise<Record<string, string>> {
	return invoke('load_cheatsheet', { files });
}

export function invokeLoadCheatsheetSectionCommand(path: string): Promise<Record<string, string>> {
	return invoke('load_cheatsheet_section', { path });
}

export function invokeLoadCheatsheetDirectoriesCommand(): Promise<Directory[]> {
	return invoke('list_cheatsheet_directories');
}

export function invokeLoadCognitioConfigCommand(): Promise<CognitioConfig> {
	return invoke('load_cognitio_config');
}

export type OnFileChanged<T> = (event: Event<T>) => void;

export function listenForFileChangedEvents(
	onFileChanged: OnFileChanged<FileChangedPayload>
): Promise<UnlistenFn> {
	return listen('file_changed', onFileChanged);
}

export function listenForCognitioConfigChangedEvents(
	onFileChanged: OnFileChanged<CognitioConfigChangedPayload>
): Promise<UnlistenFn> {
	return listen('cognitio_config_changed', onFileChanged);
}

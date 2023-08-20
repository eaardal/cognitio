import type { Directory, File, FileChangedPayload } from '$lib/models';
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

export function invokeLoadCheatsheetDirectoriesCommand(): Promise<Directory[]> {
	return invoke('list_cheatsheet_directories');
}

export type OnFileChanged = (event: Event<FileChangedPayload>) => void;

export function listenForFileChangedEvents(onFileChanged: OnFileChanged): Promise<UnlistenFn> {
	return listen('file_changed', onFileChanged);
}

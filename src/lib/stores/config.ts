import type { CognitioConfig } from '$lib/models';
import { writable } from 'svelte/store';

export const cognitioConfig = writable<CognitioConfig>();

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}

	interface Window {
		__OPEN_LINK__: (path: string, openWith?: string) => Promise<void>;
	}
}

export {};

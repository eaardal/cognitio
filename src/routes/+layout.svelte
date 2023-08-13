<script lang="ts">
	import '../app.css';
	import { open } from '@tauri-apps/api/shell';

	// By default Tauri open links inside of the Tauri app (Cognitio web view). We want to open them in the user's default browser instead.
	// Tauri's open command is used to open links in the user's default browser.
	// This solution is an adaption of https://github.com/tauri-apps/tauri/issues/4756#issuecomment-1200745849.
	// window.__OPEN_LINK__ is invoked in Marked's renderer of links - see helpers/marked.ts.
	// TypeScript support for window.__OPEN_LINK__ is defined in app.d.ts.
	window.__OPEN_LINK__ = open;
</script>

<slot />

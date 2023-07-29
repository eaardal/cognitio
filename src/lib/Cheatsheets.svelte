<script lang="ts">
	import Cheatsheet from './Cheatsheet.svelte';
	import type { Cheatsheet as CheatsheetModel } from './models';

	export let cheatsheets: CheatsheetModel[] = [];
</script>

{#each cheatsheets as cheatsheet}
	<Cheatsheet {cheatsheet} />
{/each}

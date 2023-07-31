<script lang="ts">
	import { afterUpdate } from 'svelte';
	import './Cheatsheet.css';
	export let cheatsheet: Record<string, string>;

	afterUpdate(() => {
		const divs = document.querySelectorAll('#mk-copy-btn');
		divs.forEach((div) => {
			div.addEventListener('click', (ev) => {
				const target = ev.target as HTMLButtonElement;
				const codeElems = target.parentElement?.querySelectorAll('code');

				if (codeElems && codeElems.length > 0) {
					const code = codeElems[0];
					navigator.clipboard.writeText(code.innerText);
					console.log('wrote to clipboard:', code.innerText);
				}
			});
		});
	});
</script>

<div class="cheatsheet">
	{#each Object.keys(cheatsheet) as sectionName}
		<h3 class="section-title">{sectionName}</h3>
		<div class="cheatsheet-html-root">{@html cheatsheet[sectionName]}</div>
	{/each}
</div>

<style>
	.cheatsheet {
		display: flex;
		flex-direction: column;
	}

	.cheatsheet-html-root {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}
</style>

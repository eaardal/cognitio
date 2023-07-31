<script lang="ts">
	import './Cheatsheet.css';
	export let cheatsheet: Record<string, string>;
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

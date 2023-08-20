<script lang="ts">
	export let markdown: string;
</script>

<!-- eslint-disable svelte/no-at-html-tags -->
<div class="cheatsheet-html-root">{@html markdown}</div>

<!-- eslint-enable -->

<style>
	.cheatsheet-html-root {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}
</style>

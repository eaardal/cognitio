<script lang="ts">
	import type { MenuItem } from '$lib/models';
	import MenuSubItem from './MenuSubItem.svelte';

	export let items: MenuItem[] = [];
</script>

<ul class="menu-sub-section">
	{#each items as item}
		<MenuSubItem {item} />
	{/each}
</ul>

<style>
	.menu-sub-section {
		margin-bottom: 16px;
	}
</style>

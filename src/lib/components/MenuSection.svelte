<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import orderBy from 'lodash/orderBy';
	import type { MenuItem, MenuSection } from '$lib/models';
	import MenuSubSection from './MenuSubSection.svelte';

	const dispatch = createEventDispatcher();

	export let menuSection: MenuSection | undefined;
	export let activeMenuItemId: string | undefined;
	export let showSectionTitle: boolean = false;

	function onMenuItemClick(menuItem: MenuItem): undefined {
		dispatch('menu-item-click', menuItem);
	}

	function showSubSection(menuItem: MenuItem): boolean {
		return !!activeMenuItemId && menuItem.id === activeMenuItemId && menuItem.children.length > 0;
	}

	function removeAnchorHref() {
		// Svelte has a accessibility (a11y) module that warns about a11y violations.
		// We want some anchor tags in the menu to have a on:click handler but not a href attribute,
		// but the a11y module complains about this. This function removes the href attribute while keeping a11y happy because href is set initially.
		document.querySelectorAll('a[href="[a11y-remove]"]').forEach((anchor) => {
			anchor.removeAttribute('href');
		});
	}

	onMount(() => {
		removeAnchorHref();
	});
</script>

<div class="menu-section">
	{#if showSectionTitle && menuSection}
		<li class="menu-section-title">
			<div>{menuSection.title}</div>
		</li>
	{/if}

	{#each menuSection?.items || [] as menuItem}
		<li class="menu-item" class:active={activeMenuItemId && activeMenuItemId === menuItem.id}>
			<a href="[a11y-remove]" on:click={onMenuItemClick(menuItem)}>{menuItem.title}</a>
			{#if showSubSection(menuItem)}
				<MenuSubSection items={orderBy(menuItem.children, ['title'], ['asc'])} />
			{/if}
		</li>
	{/each}
</div>

<style>
	.menu-section {
		margin-bottom: 16px;
	}

	.menu-section-title div {
		color: color-mix(in srgb, var(--foreground) 30%, var(--white));
		text-transform: uppercase;
		font-size: 0.8em;
		letter-spacing: 1.3px;
	}

	.menu-item {
		font-size: 1em;
		color: var(--white);
	}

	.menu-item > a {
		cursor: pointer;
	}

	.active > a {
		font-weight: 600;
		color: var(--white);
	}
</style>

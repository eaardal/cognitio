<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import orderBy from 'lodash/orderBy';
	import type { MenuItem, MenuSection } from '$lib/models';
	import Tooltip from './Tooltip.svelte';

	const dispatch = createEventDispatcher();

	export let menuSection: MenuSection | undefined;
	export let active: MenuItem | undefined;
	export let showSectionTitle: boolean = false;

	function onMenuItemClick(menuItem: MenuItem): undefined {
		dispatch('menu-item-click', menuItem);
	}
</script>

<div class="menu-section">
	{#if showSectionTitle && menuSection}
		<Tooltip content={menuSection.tooltipText}>
			<li class="menu-section-title">
				<div>{menuSection.title}</div>
			</li>
		</Tooltip>
	{/if}

	{#each menuSection?.items || [] as menuItem}
		<li class="menu-item" class:active={active?.id === menuItem.id}>
			<a href="" on:click={onMenuItemClick(menuItem)}>{menuItem.title}</a>
			{#if active && menuItem.id === active.id && menuItem.children.length > 0}
				<ul class="menu-section">
					{#each orderBy(menuItem.children, ['title'], ['asc']) as subMenuItem}
						<li class="menu-sub-item">
							<a href="" on:click={onMenuItemClick(subMenuItem)}>{subMenuItem.title}</a>
						</li>
					{/each}
				</ul>
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

	.menu-sub-item {
		font-size: 0.8em;
		color: var(--white);
		list-style-type: none;
		padding: 0;
		margin: 0;
	}

	.menu-sub-item a {
		color: color-mix(in srgb, var(--theme-5) 70%, var(--white));
	}

	.active > a {
		font-weight: 600;
		color: var(--white);
	}
</style>

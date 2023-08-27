<script lang="ts">
	import { onMount } from 'svelte';
	import type { MenuItem } from '$lib/models';

	export let item: MenuItem;

	let isActive = window.location.hash.includes(item.title);

	onMount(() => {
		window.addEventListener('hashchange', updateActiveState);

		return () => {
			window.removeEventListener('hashchange', updateActiveState);
		};
	});

	function updateActiveState() {
		isActive = window.location.hash.includes(encodeURIComponent(item.title));
	}
</script>

<li class="menu-sub-item" class:active={isActive}>
	<a href={`#section_${item.title}`}>{item.title}</a>
</li>

<style>
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

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Tooltip from './Tooltip.svelte';

	export let name: string;

	const dispatch = createEventDispatcher();

	function onEditClick() {
		dispatch('edit-click', { name });
	}

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}
</script>

<!-- svelte-ignore a11y-missing-content -->
<a id={name} />

<h3 class="section-title">
	{name}
	<Tooltip content="Go to the top of the page" top="120%">
		<button class="go-to-top-btn" on:click={scrollToTop}>Top</button>
	</Tooltip>
	<Tooltip content={`Edit "${name}.md"`} top="120%">
		<button class="edit-btn" on:click={() => onEditClick()}>Edit</button>
	</Tooltip>
</h3>

<style>
	.section-title {
		display: flex;
		align-items: center;
	}

	.go-to-top-btn {
		display: inline-block;
		border: none;
		padding: 2px 4px;
		margin: 0;
		text-decoration: none;
		background: var(--foreground);
		color: var(--accent);
		font-family: 'REM';
		font-size: 0.8rem;
		cursor: pointer;
		text-align: center;
		transition: background 250ms ease-in-out, transform 150ms ease;
		border-radius: 4px;
		margin-left: 16px;
	}

	.edit-btn {
		display: inline-block;
		border: none;
		padding: 2px 4px;
		margin: 0;
		text-decoration: none;
		background: var(--foreground);
		color: var(--accent);
		font-family: sans-serif;
		font-size: 0.8rem;
		cursor: pointer;
		text-align: center;
		transition: background 250ms ease-in-out, transform 150ms ease;
		border-radius: 4px;
		margin-left: 16px;
	}

	.edit-btn:hover,
	.edit-btn:focus {
		background: var(--foreground-lighter);
	}
</style>

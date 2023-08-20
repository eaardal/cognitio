<script lang="ts">
	export let content = '';
	export let top = '100%';
	export let left = '0';
	export let right = '0';
</script>

<div class="tooltip-container">
	<span class="tooltip" style="top: {top}; left: {left}; right: {right}">{content}</span>
	<slot />
</div>

<style>
	.tooltip-container {
		position: relative;
		display: inline-block;
		cursor: pointer;
	}

	.tooltip {
		display: none;
		position: absolute;
		padding: 8px;
		background-color: #333;
		width: fit-content;
		transform: translateX(0%);
		color: #fff;
		border-radius: 4px;
		font-size: 0.8em;
		white-space: nowrap;
		z-index: 1;
	}

	.tooltip-container:hover .tooltip {
		display: block;
	}
</style>

<script lang="ts">
	export let text = '';
</script>

<div class="tooltip-container">
	<span class="tooltip">{text}</span>
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
		top: 100%;
		left: 300%;
		transform: translateX(-50%);
		padding: 8px;
		background-color: #333;
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

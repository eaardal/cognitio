<script lang="ts">
	export let code: string;
	export let description: string;
</script>

<div class="root">
	<div class="row">
		<pre>
      <code>{code}</code>
    </pre>
		<div>
			{description}
		</div>
	</div>
</div>

<style>
	.root {
		border-style: solid;
		border-width: 1px;
	}

	.row {
		display: flex;
	}
</style>

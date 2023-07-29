<script lang="ts">
	export let cheatsheet: Record<string, string>;
</script>

{#each Object.keys(cheatsheet) as sectionName}
	<h3>{sectionName}</h3>
	<pre>
		<code>
			{cheatsheet[sectionName]}
		</code>
	</pre>
{/each}

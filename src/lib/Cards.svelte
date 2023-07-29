<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';
	import CodeTuple from './CodeTuple.svelte';
	import Description from './Description.svelte';
	import type { Card } from './models';
	import { isCodeBlock, isCodeTuple, isDescription } from './models';

	export let cards: Card[];
</script>

{#each cards as card}
	{#each card.contents as content}
		{#if isCodeBlock(content)}
			<CodeBlock code={content.code} />
		{/if}
		{#if isCodeTuple(content)}
			<CodeTuple code={content.code} description={content.description} />
		{/if}
		{#if isDescription(content)}
			<Description text={content.text} />
		{/if}
	{/each}
{/each}

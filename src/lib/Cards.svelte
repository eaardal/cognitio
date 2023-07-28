<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';
	import CodeTuple from './CodeTuple.svelte';
	import Description from './Description.svelte';
	import type { Card } from './models';
	import { isCodeBlock, isCodeTuple, isDescription } from './models';

	export let cards: Card[];
</script>

{#each cards as card}
	{#if isCodeBlock(card)}
		<CodeBlock code={card.code} />
	{/if}
	{#if isCodeTuple(card)}
		<CodeTuple code={card.code} description={card.description} />
	{/if}
	{#if isDescription(card)}
		<Description text={card.text} />
	{/if}
{/each}

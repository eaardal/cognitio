<script lang="ts">
	import Cards from './Cards.svelte';
	import type { Subject } from './models';

	export let subject: Subject;
</script>

<h3>{subject.title}</h3>
{#each subject.sections as section}
	<h3>{section.title}</h3>
	<Cards cards={section.cards} />
{/each}
{#each subject.chapters as chapter}
	<h2>{chapter.title}</h2>
	{#each chapter.sections as section}
		<h3>{section.title}</h3>
		<Cards cards={section.cards} />
	{/each}
{/each}

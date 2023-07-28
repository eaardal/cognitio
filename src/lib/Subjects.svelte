<script lang="ts">
	import Subject from './Subject.svelte';
	import type { Subject as SubjectModel } from './models';

	export let subjects: SubjectModel[] = [];
</script>

{#each subjects as subject}
	<Subject {subject} />
{/each}

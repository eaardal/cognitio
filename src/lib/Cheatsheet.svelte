<script lang="ts">
	import Subjects from './Subjects.svelte';
	import type {
		Subject,
		CodeBlockCard,
		CodeTupleCard,
		DescriptionCard,
		Section,
		Chapter
	} from './models';

	const conditionalExecution: Section = {
		title: 'Conditional execution',
		cards: [
			{
				kind: 'codeBlock',
				code: `git commit && git push
git commit || echo "Commit failed"`
			} as CodeBlockCard
		]
	};

	const conditionals: Section = {
		title: 'Conditionals',
		cards: [
			{
				kind: 'codeBlock',
				code: `if [[ -z "$string" ]]; then
  echo "String is empty"
elif [[ -n "$string" ]]; then
  echo "String is not empty"
fi`
			} as CodeBlockCard,
			{
				kind: 'description',
				text: 'See: Conditionals'
			} as DescriptionCard
		]
	};

	const variables: Section = {
		title: 'Variables',
		cards: [
			{
				kind: 'codeBlock',
				code: `name=\"John\"
echo $name  # see below
echo \"\$name\"
echo \"\${name}!\"`
			} as CodeBlockCard,
			{
				kind: 'description',
				text: 'Generally quote your variables unless they contain wildcards to expand or command fragments.'
			} as DescriptionCard,
			{
				kind: 'codeBlock',
				code: `wildcard="*.txt"
options="iv"
cp -$options $wildcard /tmp`
			} as CodeBlockCard
		]
	};

	const substitution: Section = {
		title: 'Substitution',
		cards: [
			{
				kind: 'codeTuple',
				code: '${foo%suffix}',
				description: 'Remove suffix'
			} as CodeTupleCard,
			{
				kind: 'codeTuple',
				code: '${foo#prefix}',
				description: 'Remove prefix'
			} as CodeTupleCard,
			{
				kind: 'codeTuple',
				code: '${foo%%suffix}',
				description: 'Remove long suffix'
			} as CodeTupleCard
		]
	};

	const parameterExpansions: Chapter = {
		title: 'Paramtere expansions',
		sections: [substitution]
	};

	const subjects: Subject[] = [
		{
			title: 'Bash scripting',
			sections: [conditionalExecution, conditionals],
			chapters: [parameterExpansions]
		}
	];
</script>

<Subjects {subjects} />

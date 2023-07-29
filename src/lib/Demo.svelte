<script lang="ts">
	import Cheatsheets from './Cheatsheets.svelte';
	import type {
		Cheatsheet as CheatsheetModel,
		CodeBlockCardContent,
		CodeTupleCardContent,
		DescriptionCardContent,
		Section,
		Chapter
	} from './models';

	const conditionalExecution: Section = {
		title: 'Conditional execution',
		cards: [
			{
				contents: [
					{
						kind: 'codeBlock',
						code: `git commit && git push
git commit || echo "Commit failed"`
					} as CodeBlockCardContent
				]
			}
		]
	};

	const conditionals: Section = {
		title: 'Conditionals',
		cards: [
			{
				contents: [
					{
						kind: 'codeBlock',
						code: `if [[ -z "$string" ]]; then
  echo "String is empty"
elif [[ -n "$string" ]]; then
  echo "String is not empty"
fi`
					} as CodeBlockCardContent
				]
			},
			{
				contents: [
					{
						kind: 'description',
						text: 'See: Conditionals'
					} as DescriptionCardContent
				]
			}
		]
	};

	const variables: Section = {
		title: 'Variables',
		cards: [
			{
				contents: [
					{
						kind: 'codeBlock',
						code: `name=\"John\"
echo $name  # see below
echo \"\$name\"
echo \"\${name}!\"`
					} as CodeBlockCardContent
				]
			},
			{
				contents: [
					{
						kind: 'description',
						text: 'Generally quote your variables unless they contain wildcards to expand or command fragments.'
					} as DescriptionCardContent
				]
			},
			{
				contents: [
					{
						kind: 'codeBlock',
						code: `wildcard="*.txt"
options="iv"
cp -$options $wildcard /tmp`
					} as CodeBlockCardContent
				]
			}
		]
	};

	const substitution: Section = {
		title: 'Substitution',
		cards: [
			{
				contents: [
					{
						kind: 'codeTuple',
						code: '${foo%suffix}',
						description: 'Remove suffix'
					} as CodeTupleCardContent
				]
			},
			{
				contents: [
					{
						kind: 'codeTuple',
						code: '${foo#prefix}',
						description: 'Remove prefix'
					} as CodeTupleCardContent
				]
			},
			{
				contents: [
					{
						kind: 'codeTuple',
						code: '${foo%%suffix}',
						description: 'Remove long suffix'
					} as CodeTupleCardContent
				]
			}
		]
	};

	const parameterExpansions: Chapter = {
		title: 'Paramtere expansions',
		sections: [substitution]
	};

	const cheatsheets: CheatsheetModel[] = [
		{
			title: 'Bash scripting',
			sections: [conditionalExecution, conditionals],
			chapters: [parameterExpansions]
		}
	];
</script>

<Cheatsheets {cheatsheets} />

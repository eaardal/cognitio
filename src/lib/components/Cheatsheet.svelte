<script lang="ts">
	import { afterUpdate, createEventDispatcher } from 'svelte';
	import '$lib/components/Cheatsheet.css';

	export let cheatsheet: Record<string, string>;
	export let name: string;
	export let path: string;

	const codeBgOnCopyHover = '#584796';
	const codeBgDefault = 'transparent';

	const dispatch = createEventDispatcher();

	function onEditCheatsheetClick() {
		dispatch('edit-cheatsheet', { path, name });
	}

	function getLastPartSeparatedByDash(inputString: string): string {
		const parts = inputString.split('-');
		return parts[parts.length - 1];
	}

	function findCodeBoundaryForCopyButton(btn: Element): HTMLElement | null {
		const id = getLastPartSeparatedByDash(btn.id);
		return document.getElementById(`mk-code-boundary-${id}`);
	}

	afterUpdate(() => {
		const buttons = document.querySelectorAll('.mk-copy-btn');
		buttons.forEach((button) => {
			button.addEventListener('click', (ev) => {
				const target = ev.target as HTMLButtonElement;
				if (!target) {
					return;
				}

				const codeElems = target.parentElement?.querySelectorAll('code');

				if (codeElems && codeElems.length > 0) {
					const code = codeElems[0];
					navigator.clipboard.writeText(code.innerText);

					const codeBoundaryDiv = findCodeBoundaryForCopyButton(target);
					if (codeBoundaryDiv) {
						codeBoundaryDiv.style.backgroundColor = 'green';
						codeBoundaryDiv.style.transitionProperty = 'background-color';
						codeBoundaryDiv.style.transitionDuration = '500ms';
						codeBoundaryDiv.style.transitionTimingFunction = 'cubic-bezier(0,1.5,0.5,1)';
						codeBoundaryDiv.style.borderStyle = 'none';
					}

					setTimeout(() => {
						const codeBoundaryDiv = findCodeBoundaryForCopyButton(target);
						if (codeBoundaryDiv) {
							codeBoundaryDiv.style.backgroundColor = '';
							codeBoundaryDiv.style.transitionProperty = '';
							codeBoundaryDiv.style.transitionDuration = '';
							codeBoundaryDiv.style.transitionTimingFunction = '';
						}
					}, 500);
				}
			});

			button.addEventListener('mouseover', () => {
				const target = findCodeBoundaryForCopyButton(button);
				if (!target) {
					return;
				}

				target.style.backgroundColor = codeBgOnCopyHover;
				target.style.borderStyle = 'dashed';
				target.style.borderWidth = '1px';
			});

			button.addEventListener('mouseout', () => {
				const target = findCodeBoundaryForCopyButton(button);
				if (!target) {
					return;
				}

				target.style.backgroundColor = codeBgDefault;
				target.style.borderStyle = 'none';
				target.style.borderWidth = '0';
			});
		});
	});
</script>

<div class="cheatsheet">
	<div class="header">
		<h2>{name}</h2>
		<button class="edit-btn" on:click={onEditCheatsheetClick}>Edit</button>
	</div>
	<div class="content">
		{#each Object.keys(cheatsheet) as sectionName}
			<h3 class="section-title">{sectionName}</h3>

			<!-- eslint-disable svelte/no-at-html-tags -->
			<div class="cheatsheet-html-root">{@html cheatsheet[sectionName]}</div>
			<!-- eslint-enable -->
		{/each}
	</div>
</div>

<style>
	.cheatsheet {
		display: flex;
		flex-direction: column;
	}

	.cheatsheet-html-root {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}

	.header {
		--padding-sides: 24px;
		background-color: color-mix(in srgb, var(--background) 90%, var(--white));
		width: calc(100%-var(--padding-sides));
		height: 50px;
		padding-left: var(--padding-sides);
		padding-right: var(--padding-sides);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.header > h2 {
		font-size: 1.2em;
		margin: 0;
		padding: 0;
		color: var(--theme-3);
	}

	.content {
		margin-left: 24px;
	}

	.edit-btn {
		display: inline-block;
		border: none;
		padding: 2px 4px;
		margin: 0;
		text-decoration: none;
		background: var(--foreground);
		color: var(--accent);
		font-family: sans-serif;
		font-size: 0.8rem;
		cursor: pointer;
		text-align: center;
		transition: background 250ms ease-in-out, transform 150ms ease;
		border-radius: 4px;
		margin-left: 16px;
	}

	.edit-btn:hover,
	.edit-btn:focus {
		background: var(--foreground-lighter);
	}
</style>

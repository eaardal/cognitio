<script lang="ts">
	import { afterUpdate } from 'svelte';
	import './Cheatsheet.css';

	export let cheatsheet: Record<string, string>;

	const codeBgOnCopyHover = '#584796';
	const codeBgDefault = 'transparent';

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
	{#each Object.keys(cheatsheet) as sectionName}
		<h3 class="section-title">{sectionName}</h3>
		<div class="cheatsheet-html-root">{@html cheatsheet[sectionName]}</div>
	{/each}
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
</style>

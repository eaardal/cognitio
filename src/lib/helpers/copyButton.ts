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

function onClick(ev: Event) {
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
}

function onMouseOver(button: Element) {
	return () => {
		const target = findCodeBoundaryForCopyButton(button);
		if (!target) {
			return;
		}

		// Styling here must match up with base styling for .mk-code-boundary in Cheatsheet.css
		target.style.backgroundColor = codeBgOnCopyHover;
		target.style.borderStyle = 'dashed';
		target.style.borderWidth = '1px';
		target.style.borderColor = '#edded8';
		target.style.width = 'calc(100% - 2px)'; // Allow space for border without causing a horizontal scrollbar
	};
}

function onMouseOut(button: Element) {
	return () => {
		const target = findCodeBoundaryForCopyButton(button);
		if (!target) {
			return;
		}

		// Styling here must match up with base styling for .mk-code-boundary in Cheatsheet.css
		target.style.backgroundColor = codeBgDefault;
		target.style.borderStyle = 'solid';
		target.style.borderWidth = '1px';
		target.style.borderColor = 'color(srgb 0.20666668 0.16745098 0.3482353)'; // matches var(--background-lighter)
		target.style.width = 'calc(100%-2px)'; // Allow space for border without causing a horizontal scrollbar
	};
}

function addEventHandlersToButton(button: Element) {
	button.addEventListener('click', onClick);
	button.addEventListener('mouseover', onMouseOver(button));
	button.addEventListener('mouseout', onMouseOut(button));
}

export function addEventHandlersToCopyButtons() {
	const buttons = document.querySelectorAll('.mk-copy-btn');
	buttons.forEach((button) => {
		addEventHandlersToButton(button);
	});
}

// Generate a 6 character string of random letters and numbers we can use as a unique ID.
function generateRandomText(): string {
	const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	let result = '';
	const charactersLength = characters.length;

	for (let i = 0; i < 6; i++) {
		const randomIndex = Math.floor(Math.random() * charactersLength);
		result += characters.charAt(randomIndex);
	}

	return result;
}

// Walk backwards from each <pre> tag until we meet the section's heading (<h3>) or another <pre> tag
// and wrap everything in between into a new div.
function walkBackwards(preTags: NodeListOf<HTMLPreElement>, doc: Document) {
	preTags.forEach((pre) => {
		// Create a new div with class "mk-text-block"
		const newDiv = doc.createElement('div');
		newDiv.className = 'mk-text-block';

		// Traverse backwards from the current pre tag
		let prevSibling = pre.previousElementSibling;
		while (
			prevSibling &&
			!(prevSibling.tagName.toLowerCase() === 'h3' || prevSibling.tagName.toLowerCase() === 'pre')
		) {
			const prevElement = prevSibling.previousElementSibling;
			// Move the current previous sibling into the new div
			newDiv.insertBefore(prevSibling, newDiv.firstChild);
			prevSibling = prevElement;
		}

		// If we found any elements to move, insert the new div into the DOM.
		if (newDiv.children.length > 0) {
			// Insert the new div before the current pre tag
			pre.parentNode?.insertBefore(newDiv, pre);
		}
	});
}

// Walk forwards from each <pre> tag until we meet another <pre> tag
// and wrap everything in between into a new div.
function walkForwards(preTags: NodeListOf<HTMLPreElement>, doc: Document) {
	preTags.forEach((pre) => {
		// Create a new div with class "mk-text-block"
		const newDiv = doc.createElement('div');
		newDiv.className = 'mk-text-block';

		// Traverse forward from the current pre tag
		let nextSibling = pre.nextElementSibling;
		while (nextSibling && !(nextSibling.tagName.toLowerCase() === 'pre')) {
			const nextElement = nextSibling.nextElementSibling;
			// Move the current next sibling into the new div
			newDiv.appendChild(nextSibling);
			nextSibling = nextElement;
		}

		if (newDiv.children.length > 0) {
			// Insert the new div after the current pre tag
			pre.parentNode?.insertBefore(newDiv, pre.nextSibling);
		}
	});
}

function makeCodeBoundaryDiv(doc: Document, id: string): HTMLDivElement {
	const codeBoundaryDiv = doc.createElement('div');
	codeBoundaryDiv.id = `mk-code-boundary-${id}`;
	codeBoundaryDiv.className = 'mk-code-boundary';
	return codeBoundaryDiv;
}

function makeCopyButton(doc: Document, id: string): HTMLInputElement {
	const copybtn = doc.createElement('input');
	copybtn.id = `mk-copy-btn-${id}`;
	copybtn.className = 'mk-copy-btn';
	copybtn.type = 'button';
	copybtn.value = 'Copy';
	return copybtn;
}

function makeCodeBlockDiv(doc: Document): HTMLDivElement {
	const codeBlockDiv = doc.createElement('div');
	codeBlockDiv.className = 'mk-code-block';
	return codeBlockDiv;
}

function ensureNoMkTextBlockSiblings(doc: Document) {
	const textBlocks = doc.querySelectorAll('.mk-text-block');
	const textBlocksArray = Array.from(textBlocks);

	function isParent(element: Element, potentialParent: Element) {
		return element.contains(potentialParent);
	}

	textBlocksArray.forEach((textBlock, index) => {
		// Check if it has a parent with class mk-text-block
		const mkTextBlockParent = textBlocksArray.find(
			(tb, i) => i !== index && isParent(tb, textBlock)
		);

		if (mkTextBlockParent) {
			// If it has such a parent, lift the child up and replace the parent
			mkTextBlockParent.parentNode?.replaceChild(textBlock, mkTextBlockParent);
		}
	});
}

/**
 * To make each cheatsheet section as easy to read as possible, we want to wrap
 * all content like <p> and <li> tags in divs that we can target with styling:
 *
 * <div class="mk-text-block">
 *   ...all p tags
 *   ...all li tags
 *   ...etc
 * </div>
 * <pre><code></code></pre>
 *
 * This would be rather simple and could be done much like with the custom wrapper <div class="mk-section">
 * as described in markdown.ts addCustomCssClassesToMarkdown.heading function's comments if only
 * marked had a hook for manipulating <pre> tags like they do for "headings", "paragraphs", "code" and more (see markdown.ts).
 *
 * Unfortunately, they don't have such a hook for <pre> tags so we need to find these ourselves
 * once the markdown has been processed to HTML and manipulate the final HTML ourselves to wrap
 * the content we want in mk-text-block divs.
 *
 * patchHtmlWithMkTextBlockDivs takes care of this mess...
 */
export function patchHtmlWithMkTextBlockDivs(html: string) {
	const parser = new DOMParser();
	const doc = parser.parseFromString(html, 'text/html');

	// The logic for wrapping content in mk-text-block divs is to find all <pre> tags
	// and walk the sibling elements backwards until we meet a <h3> or <pre> tag.
	// Every HTML element we found along the way should be added to a new div with the class mk-text-block.
	//
	// Example:
	//
	// This HTML:
	//
	// <h3>JavaScript functions</h3>
	// <p>Some text</p>
	// <p>More text</p>
	// <ul>
	//   <li>Remember this</li>
	//   <li>And this</li>
	// </ul>
	// <pre>
	//   <code>...code example</code>
	// </pre>
	//
	// Should be converted to this:
	//
	// <h3>JavaScript functions</h3>
	// <div class="mk-text-block">
	//   <p>Some text</p>
	//   <p>More text</p>
	//   <ul>
	//     <li>Remember this</li>
	//     <li>And this</li>
	//   </ul>
	// </div>
	// <pre>
	//   <code>...code example</code>
	// </pre>
	//
	// <pre> tags acts as stop-points. mk-text-block divs should end directly before <pre> tags or start directly after them.

	// Find all the <pre> tags in the document:
	const preTags = doc.querySelectorAll('pre');

	// From each <pre> tag, walk backwards and insert necessary divs with the .mk-text-block class.
	walkBackwards(preTags, doc);

	// Then walk forwards and do the same.
	walkForwards(preTags, doc);

	preTags.forEach((pre) => {
		const elementId = generateRandomText();

		// Ensure that all <pre> tags has a .mk-pre class we can target with styling (see Cheatsheet.css).
		pre.classList.add('mk-pre');
		// We need to know which <pre> tag we're dealing with when we want to copy the code to the clipboard, so make sure a random ID for this particular <pre> is part of its ID.
		pre.id = `mk-pre-${elementId}`;

		// Wrap the <pre> tag in a new div with class "mk-code-block" so we can target it with styling and position it correctly.
		const codeBlockDiv = makeCodeBlockDiv(doc);
		codeBlockDiv.className = 'mk-code-block';

		pre.parentNode?.insertBefore(codeBlockDiv, pre);

		// Insert a new div with class "mk-code-boundary" before the <pre> tag, but inside the mk-code-block div.
		const codeBoundaryDiv = makeCodeBoundaryDiv(doc, elementId);
		codeBoundaryDiv.appendChild(pre);
		codeBlockDiv.appendChild(codeBoundaryDiv);

		// Add a <button> with class "mk-copy-btn" that we later can use to copy the code to the clipboard. This is part of the mk-code-boundary div.
		const copybtn = makeCopyButton(doc, elementId);
		codeBlockDiv.appendChild(copybtn);
	});

	ensureNoMkTextBlockSiblings(doc);

	// Serialize the updated document back to an HTML string
	return new XMLSerializer().serializeToString(doc);
}

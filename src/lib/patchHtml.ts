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
 * patchHtmlWithMkTextBlockDivs ptakes care of this mess...
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

	// <pre> tags acts as stop-points. mk-text-block divs should end directly before <pre> tags or start directly after them.
	// Find all the <pre> tags in the document:
	const preTags = doc.querySelectorAll('pre');

	walkBackwards(preTags, doc);
	walkForwards(preTags, doc);

	preTags.forEach((pre) => {
		pre.classList.add('mk-pre');

		const div = doc.createElement('div');
		div.className = 'mk-code-block';

		pre.parentNode?.insertBefore(div, pre);
		div.appendChild(pre);

		const copybtn = doc.createElement('input');
		copybtn.type = 'button';
		copybtn.value = 'Copy';
		copybtn.id = 'mk-copy-btn';
		copybtn.className = 'mk-copy-btn';

		div.appendChild(copybtn);
	});

	// Serialize the updated document back to an HTML string
	return new XMLSerializer().serializeToString(doc);
}

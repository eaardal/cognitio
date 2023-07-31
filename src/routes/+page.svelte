<script lang="ts">
	import 'highlight.js/styles/github-dark.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Renderer, marked } from 'marked';
	import { markedHighlight } from 'marked-highlight';
	import dompurify from 'dompurify';
	import { parse, flatMap, traverseDescendantNodes, stringify } from 'extra-dom';
	import hljs from 'highlight.js';
	import Cheatsheet from '$lib/Cheatsheet.svelte';
	import Menu from '$lib/Menu.svelte';
	import type { Directory, File } from '$lib/models';

	hljs.highlightAll();

	let isSectionStarted = false;
	// let isTextStarted = false;

	const renderer = {
		heading(text: string, level: number) {
			let wrapper = '';
			// if (isTextStarted) {
			// 	wrapper += `</div>`;
			// 	isTextStarted = false;
			// }
			if (level === 3 && isSectionStarted) {
				wrapper += `</div>`;
				isSectionStarted = false;
			}
			if (level === 3 && !isSectionStarted) {
				isSectionStarted = true;
				wrapper += `<div class="mk-section">`;
			}
			wrapper += `<h${level} class="mk-h${level}">${text}</h${level}>`;
			return wrapper;
		},
		paragraph(text: string) {
			let wrapper = '';
			// if (isTextStarted) {
			// 	wrapper += `</div>`
			// 	isTextStarted=false;
			// }
			// if (!isTextStarted) {
			// 	isTextStarted = true;
			// 	wrapper += `<div class="mk-text-block">`;
			// }
			wrapper += `<p class="mk-p">${text}</p>`;
			return wrapper;
		},
		code(code: string, language: string, isEscaped: boolean) {
			let wrapper = '';
			// if (isTextStarted) {
			// 	wrapper += `</div>`;
			// 	isTextStarted = false;
			// }
			wrapper += `<code class="mk-code language-${language}">${code}</code>`;
			return wrapper;
		},
		codespan(code: string) {
			// Håndterer inline code: `const foo = "hei"`
			console.log('codespan:', code);
			return code;
		}
	};

	function fixHtml(html: string) {
		const parser = new DOMParser();
		const doc = parser.parseFromString(html, 'text/html');

		// Get all pre elements
		const preTags = doc.querySelectorAll('pre');

		// Loop through each pre tag
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

			if (newDiv.children.length > 0) {
				// Insert the new div before the current pre tag
				pre.parentNode?.insertBefore(newDiv, pre);
			}
		});

		// Get all p elements
		const pTags = doc.querySelectorAll('p');

		// Loop through each p tag
		pTags.forEach((p) => {
			// Check if the p tag has no siblings after it
			if (!p.nextElementSibling) {
				// Create a new div with class "mk-text-block"
				const newDiv = doc.createElement('div');
				newDiv.className = 'mk-text-block';

				// Clone the p tag and append it to the new div
				const clonedP = p.cloneNode(true);
				newDiv.appendChild(clonedP);

				// Replace the current p tag with the new div
				p.parentNode?.replaceChild(newDiv, p);
			}
		});

		// Serialize the updated document back to an HTML string
		const updatedHtml = new XMLSerializer().serializeToString(doc);
		return updatedHtml;
	}

	const hooks = {
		postprocess(html: string) {
			console.log('html', html);
			const fixed = fixHtml(html);
			console.log('fixed', fixed);
			return fixed;
		}
	};

	marked.use(
		{
			mangle: false,
			headerIds: false,
			renderer,
			hooks
		},
		markedHighlight({
			highlight(code, lang) {
				const language = hljs.getLanguage(lang) ? lang : 'plaintext';
				return hljs.highlight(code, { language }).value;
			}
		})
	);

	let currentCheatsheet: Record<string, string>;

	function loadCheatsheet(files: File[]) {
		invoke('load_cheatsheet', { files })
			.then((value) => {
				const rawCheatsheet = value as Record<string, string>;

				const cheatsheet = Object.keys(rawCheatsheet).reduce((all, key) => {
					const markdown = rawCheatsheet[key] as string;
					const sanitizedMarkdown = dompurify.sanitize(markdown);
					const html = marked.parse(sanitizedMarkdown);
					return html ? { ...all, [key]: html } : all;
				}, {} as Record<string, string>);

				currentCheatsheet = cheatsheet;
			})
			.catch((e) => {
				console.error(e);
			});
	}

	function menuItemClicked(event: CustomEvent<Directory>) {
		loadCheatsheet(event.detail.files);
	}
</script>

<div class="page-root">
	<Menu on:onMenuItemClick={menuItemClicked} />

	<div class="page-content">
		{#if currentCheatsheet}
			<Cheatsheet cheatsheet={currentCheatsheet} />
		{/if}
	</div>
</div>

<style>
	.page-root {
		height: 100%;
		display: flex;
		flex-direction: row;
	}

	.page-content {
		padding: 24px;
	}
</style>

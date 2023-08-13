import 'highlight.js/styles/github-dark.css';
import { marked } from 'marked';
import { markedHighlight } from 'marked-highlight';
import hljs from 'highlight.js';
import { open } from '@tauri-apps/api/shell';
import { patchHtmlWithMkTextBlockDivs } from './patchHtml';

// Enable syntax highlighting
hljs.highlightAll();

// To wrap HTML tags inside a new div, we use this flag to track wether we
// have opened the HTML tag (we have inserted a <div> but not closed it yet with </div>).
let isMkSectionDivOpen = false;

const isH3Tag = (level: number) => level === 3;

const addCustomCssClassesToMarkdown = {
	heading(text: string, level: number) {
		let html = '';

		// To style each cheatsheet section correctly as cards, we need to detect
		// where each section's content starts and ends.
		//
		// To track this, we're using <h3> tags. Whenever we see a <h3> tag, we start
		// a new section by wrapping the following tags in a div with the class "mk-section": <div class="mk-section">{all section content}</div>
		//
		// This also means that when we see a <h3> tag we need to check if a mk-section div is already open and close it (end the previous section and start a new one).
		//
		// Example:
		//
		// This markdown:
		//
		// ### JavaScript cheats
		//
		// ```javascript
		// const hello = "world";
		// ```
		//
		// ### Go cheats
		//
		// ```go
		// hello := "world"
		// ```
		//
		// Should produce the following HTML:
		//
		// <div class="mk-section">
		//   <h3>JavaScript cheats</h3>
		//   <pre>...</pre>
		// </div>
		// <div class="mk-section">
		//   <h3>Go cheats</h3>
		//   <pre>...</pre>
		// </div>

		// If a div like <div class="mk-section"> is currently open, close it.
		if (isH3Tag(level) && isMkSectionDivOpen) {
			html += `</div>`;
			isMkSectionDivOpen = false;
		}

		// If a div like <div class="mk-section"> is not open, start a new one.
		if (isH3Tag(level) && !isMkSectionDivOpen) {
			isMkSectionDivOpen = true;
			html += `<div class="mk-section">`;
		}

		// Append the heading with the custom mk-h{level} class so we can target it with styling.
		html += `<h${level} class="mk-h${level}">${text}</h${level}>`;
		return html;
	},
	paragraph(text: string) {
		return `<p class="mk-p">${text}</p>`;
	},
	code(code: string, language: string /*, escaped: boolean */) {
		return `<code class="hljs language-${language}">${code}</code>`;
	},
	codespan(code: string) {
		return `<code class="mk-inline-code">${code}</code>`;
	},
	table(header: string, body: string) {
		return `<table class="mk-table"><thead class="mk-table-head">${header}</thead><tbody class="mk-table-body">${body}</tbody></table>`;
	},
	tablerow(content: string) {
		return `<tr class="mk-table-row">${content}</tr>`;
	},
	tablecell(content: string, flags: Record<string, unknown>) {
		const align = flags.align ? `text-align: ${flags.align};` : '';
		return `<td class="mk-table-cell" style="${align}">${content}</td>`;
	},
	hr() {
		return `<hr class="mk-hr">`;
	},
	link(href: string, title: string, text: string) {
		// See setup and explanation of window.__OPEN_LINK__ in routes/+layout.svelte.
		return `<a href="#" title="${title}" class="mk-link" onclick="window.__OPEN_LINK__('${href}');">${text}</a>`;
	}
};

const hooks = {
	postprocess(html: string) {
		return patchHtmlWithMkTextBlockDivs(html);
	}
};

marked.use(
	{
		mangle: false,
		headerIds: false,
		renderer: addCustomCssClassesToMarkdown,
		hooks
	},
	markedHighlight({
		langPrefix: 'mk-code language-',
		highlight(code, lang) {
			const language = hljs.getLanguage(lang) ? lang : 'plaintext';
			return hljs.highlight(code, { language }).value;
		}
	})
);

export { marked };

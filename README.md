# Cognitio

Developer cheatsheets and code snippets via Markdown.

## Developing

Run local dev server:

```
npm run tauri dev
```

Show browser dev tools in running app:

```
CMD+OPT+I
```

### Misc

Generate app icon:

```
npm run tauri icon ./logo/logo.png
```

Build distributable:

```
npm run tauri build
```

### Scaffolding used to create project:

https://tauri.app/v1/guides/getting-started/setup/sveltekit

- With TypeScript.
- With Prettier and ESLint.
- With Vitest for testing but without browser testing.

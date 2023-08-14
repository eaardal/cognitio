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

Builds are then located under `./src-tauri/target/release/bundle`.

### Scaffolding used to create project:

https://tauri.app/v1/guides/getting-started/setup/sveltekit

- With TypeScript.
- With Prettier and ESLint.
- With Vitest for testing but without browser testing.

# TODO:

https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html#log-messages-to-a-custom-location

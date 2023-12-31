# Cognitio

Developer cheatsheets and code snippets via Markdown.

![overview](./docs/img/overview.png)

The purpose of this project is to write and maintain code snippets, cheatsheets and other developer notes in plain markdown, while viewing, looking up and browsing the content in a more delightful UI.

It also supports multiple sources of cheatsheets, making it easy for your team members or other people to make their snippets available, for example in GitHub, and importing those into your library.

:warning: Status: Alpha, see known bugs.

### Known bugs

- Detecting file changes and automatically reloading cheatsheets seems flaky when the app is built for distribution but works fine during development. Have to right click and use Reload manually to refresh for now.
- Some characters like `<` are not rendered correctly in markdown code blocks.

`IFS=';' read -a parts <<< "${userAndRole}"` gets rendered like `IFS=';' read -a parts &lt;&lt;&lt; "${userAndRole}"`

## How to get started

### Setup

1. Create the app's config file `cognitio.yaml` somewhere on your machine.

```yaml
editor: code
cheatsheets:
  - path/to/cheatsheet/dir
```

- `editor` is the command to invoke when the "Edit" or "Edit Cognitio Config" buttons are used. In the example above, Visual Studio Code is used (similar to using `code .` on the terminal).
- `cheatsheets` is a list of paths to where your snippets or cheatsheets are stored (mine can be found [here](https://github.com/eaardal/cheatsheets-and-snippets)).

2. Set the `COGNITIO_HOME` environment variable to be the path to the directory where `cognitio.yaml` is stored.

### Writing content

Use directories to make your menu and files to make sections:

![file structure](./docs/img/file-structure.png)

Section each snippet with h3 (`###`) headings and write text and code as normal:

![markdown](./docs/img/markdown.png)

All content that is not h3 headings or code blocks are shown in a darker blue/purple section in each card. This can be used to make sections inside each card.

## Developing

Run local dev server:

```
npm run tauri dev
```

Show browser dev tools in running app:

```
CMD+OPT+I
```

Build distributables (MacOS for now):

```
make build
```

### Misc

Generate app icon:

```
npm run tauri icon ./logo/logo.png
```

Build distributable (for your current OS):

```
npm run tauri build
```

Builds are then located under `./src-tauri/target/release/bundle`.

### Scaffolding used to create project:

https://tauri.app/v1/guides/getting-started/setup/sveltekit

- With TypeScript.
- With Prettier and ESLint.
- With Vitest for testing but without browser testing.

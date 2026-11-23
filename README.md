# `zcast-interfaces`

Versioned Rust interface contracts and core `Note` primitive for the [`zcast`](https://github.com/zcast-cms/zcast) modular Zettelkasten CMS.

Modules and themes depend **only** on `zcast-interfaces`—never on `zcast-kernel`—so upgrades to the CMS core keep compatible modules working without changes.

## Included Contracts (`v0.1`)
- **`Note` & `ReadContext`**: Minimal core primitive (`id`, `slug`, `title`, `markdown_body`, `private`, `aliases`, `ext`, timestamps) and mandatory visibility gate.
- **`Storage` & `ModuleStore`**: Note persistence, atomic rename with alias recording, and DB-agnostic namespaced KV/document storage for modules.
- **`LinkParser`**: Markdown rendering with `[[wikilink]]` resolution, private/unresolved hiding, backlink graph, and rename rewriting.
- **`ThemeProvider`**: Template-engine-agnostic SSR page/card rendering, light/dark/auto colour locking, UI slots, and embedded assets.
- **`EditorProvider`**: Lazy-loaded admin inline editing UI slots and pre-bundled assets.
- **`SearchProvider`** & **`AuthProvider`**: Contracts ready for `v0.2`+ discovery and authentication modules.
- **`HtmlRenderHook`** & **`NoteLifecycleHook`**: Deterministic staged hooks (`Early`, `Normal`, `Late`).

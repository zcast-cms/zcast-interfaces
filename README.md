# `zcast-interfaces`

Versioned Rust interface contracts and core `Note` primitive for the [`zcast`](https://github.com/zcast-cms/zcast) modular Zettelkasten CMS.

Modules and themes depend **only** on `zcast-interfaces`—never on `zcast-kernel`—so upgrades to the CMS core keep compatible modules working without changes.

## Included Contracts (`v0.3.0`)

- **`Note` & `ReadContext`**: Minimal core primitive (`id`, `slug`, `title`, `markdown_body`, `private`, `aliases`, `ext`, timestamps) and mandatory visibility gate.
- **`Storage` & `ModuleStore`**: Note persistence, atomic rename with alias recording, and DB-agnostic namespaced KV/document storage (with TTL & prefix queries) for modules.
- **`LinkParser`**: Markdown rendering with `[[wikilink]]` resolution, private/unresolved hiding, backlink graph, and rename rewriting.
- **`ThemeProvider`**: Template-engine-agnostic SSR page/card rendering, light/dark/auto colour locking, UI slots, and embedded assets.
- **`EditorProvider`**: Lazy-loaded admin inline editing UI slots and pre-bundled assets.
- **`UiExtension`**: Pluggable frontend UI module contract (`zcast-popover`, `zcast-cmd-k`) contributing slot HTML and embedded assets mounted under `/_zcast/ext/<prefix>/`.
- **`SearchProvider`**: Instant keyword/topical/graph-boosted search contract (`zcast:search/provider@1.0`) with strict `ReadContext` visibility enforcement.
- **`AuthProvider`**: Optional authentication contract (`zcast:auth/provider@1.0`) supporting password login/logout and scanner-safe `GET` → `POST` one-time magic links.
- **`NoteExporter` & `NoteImporter`**: Markdown directory and `.zip` archive export/import contracts (`zcast:io/markdown@1.0`).
- **`HtmlRenderHook`** & **`NoteLifecycleHook`**: Deterministic staged hooks (`Early`, `Normal`, `Late`).

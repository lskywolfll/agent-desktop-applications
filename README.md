# agent-desktop-applications

Desktop application automation CLI for AI agents.

Inspired by [agent-browser](https://github.com/vercel-labs/agent-browser), but for native desktop applications instead of the browser. An AI agent can list windows, inspect UI elements, click, type, press keys, and take screenshots — all from the terminal.

## Architecture

Client-daemon architecture (Rust + Node.js):

1. **Rust CLI** (`agent-app`) — Fast native binary that handles desktop automation via Win32 API + enigo
2. **Node.js wrapper** — npm package that downloads/installs the native binary

## Commands (planned)

| Command | Description |
|---------|-------------|
| `agent-app windows` | List all visible windows |
| `agent-app snapshot <window>` | Get accessibility tree of a window |
| `agent-app click <x> <y>` | Click at coordinates |
| `agent-app type <text>` | Type text |
| `agent-app press <key>` | Press a key |
| `agent-app screenshot <path>` | Capture screenshot |
| `agent-app install` | Download dependencies |

## Development Status

- [ ] **v0.0.1** — Placeholder published on npm
- [ ] Rust CLI: `windows` command — list windows via Win32 API
- [ ] Rust CLI: `snapshot` command — accessibility tree
- [ ] Rust CLI: `click` command — mouse click via `enigo`
- [ ] Rust CLI: `type` command — type text via `enigo`
- [ ] Rust CLI: `press` command — key press via `enigo`
- [ ] Rust CLI: `screenshot` command — capture via `screenshots` crate
- [ ] Node.js installer — `agent-app install` command
- [ ] CI — build + package native binary per platform
- [ ] SKILL.md — AI agent skill file for coding assistants

## License

GPL-3.0

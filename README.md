# 💨 CodeFart

*Your AI finished thinking. It will let you know — loudly.*

CodeFart plays a sound notification when Claude Code finishes responding. Built on Claude's native hook system — zero wrapping, zero interception.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/Onion-L/codefart/main/install.sh | bash
```

That's it. One command installs the binary, registers the Claude hook, and you're done.

## How it works

CodeFart uses Claude Code's built-in [Stop hook](https://docs.anthropic.com/en/docs/claude-code/hooks). Claude itself calls `codefart play` — CodeFart never touches your Claude process.

```
You → claude "write a function"
  → Claude thinks...
  → Claude responds
  → Claude fires Stop hook → codefart play → 💨
```

## Usage

```bash
codefart play                     # Play current sound
codefart list                     # List built-in themes
codefart theme thunder            # Switch theme
codefart set-sound ~/boom.mp3     # Use your own sound
codefart reset                    # Back to default
codefart run -- npm run build     # Wrap any command
```

## Built-in themes

| Theme | Description |
|---|---|
| `classic` | The signature CodeFart sound |
| `wet` | A wetter, more dramatic variant |

Add your own with `codefart set-sound <path>`.

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/Onion-L/codefart/main/uninstall.sh | bash
```

## License

MIT

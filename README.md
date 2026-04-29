# 💨 CodeFart

*Your AI finished thinking. It will let you know — loudly.*

<p align="center">
  <img src="https://raw.githubusercontent.com/Onion-L/codefart/main/og-image.png" alt="CodeFart" width="600" />
</p>

CodeFart plays a fart sound when Claude Code finishes responding. Built on Claude's native hook system — zero wrapping, zero interception, zero trust issues.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/Onion-L/codefart/main/install.sh | bash
codefart setup
```

Install the binary, then one command to enable Claude notifications. Done.

## How it works

CodeFart uses Claude Code's built-in [Stop hook](https://docs.anthropic.com/en/docs/claude-code/hooks). Claude itself calls `codefart play` — CodeFart never touches your Claude process, stdin, stdout, or stderr.

```
You → claude "write a function"
  → Claude thinks...
  → Claude responds
  → Claude fires Stop hook → codefart play → 💨
```

## Usage

```bash
codefart setup                   # Enable Claude notifications (once)
codefart play                    # Play current sound
codefart list                    # List themes, show current
codefart theme                   # Interactive theme picker
codefart theme classic           # Switch directly
codefart set-sound ~/boom.mp3    # Use custom audio file
codefart remove                  # Remove custom sound, back to theme
codefart reset                   # Reset everything to default
codefart update                  # Self-update to latest release
codefart run -- npm run build    # Wrap any command, fart when done
```

## Built-in themes

| Theme | Description |
|---|---|
| `classic` | The signature CodeFart sound |
| `wet` | A wetter, more dramatic variant |

Add your own with `codefart set-sound <path>`.

## Update

```bash
codefart update
```

Pulls the latest release and replaces itself. Auto-elevates with sudo when needed.

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/Onion-L/codefart/main/uninstall.sh | bash
```

Removes the binary, config, and Claude hook.

## License

MIT

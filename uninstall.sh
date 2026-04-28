#!/bin/bash
set -e

BIN_PATHS=(
    "$HOME/.local/bin/codefart"
    "/usr/local/bin/codefart"
)
CONFIG_DIR="$HOME/.config/codefart"
SETTINGS="$HOME/.claude/settings.json"

echo "Uninstalling CodeFart..."

# Remove binary
removed_binary=false
for BIN_PATH in "${BIN_PATHS[@]}"; do
    if [ -f "$BIN_PATH" ]; then
        rm -f "$BIN_PATH" 2>/dev/null || sudo rm -f "$BIN_PATH"
        echo "✓ Removed $BIN_PATH"
        removed_binary=true
    fi
done

if [ "$removed_binary" = false ]; then
    echo "  Binary not found"
fi

# Remove config
if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
    echo "✓ Removed $CONFIG_DIR"
fi

# Remove hook from Claude settings
if [ -f "$SETTINGS" ]; then
    python3 -c "
import json
with open('$SETTINGS') as f:
    s = json.load(f)
hooks = s.get('hooks', {})
stop = hooks.get('Stop', [])
before = len(stop)
hooks['Stop'] = [h for h in stop if not any(
    hh.get('command') == 'codefart play'
    for hh in h.get('hooks', [])
)]
if not hooks['Stop']:
    del hooks['Stop']
if not hooks:
    del s['hooks']
after = len(hooks.get('Stop', []))
with open('$SETTINGS', 'w') as f:
    json.dump(s, f, indent=2)
print(f'✓ Removed {before - after} CodeFart hook(s) from Claude settings')
" 2>/dev/null || echo "  Could not clean Claude settings (non-critical)"
fi

echo ""
echo "CodeFart uninstalled."

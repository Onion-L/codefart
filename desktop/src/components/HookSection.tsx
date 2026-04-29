import { useState } from "react";

interface HookSectionProps {
    installed: boolean;
    onInstall: () => void | Promise<void>;
    onUninstall: () => void | Promise<void>;
}

export default function HookSection({ installed, onInstall, onUninstall }: HookSectionProps) {
    const [busy, setBusy] = useState(false);

    const handle = async (fn: () => void | Promise<void>) => {
        setBusy(true);
        try {
            await fn();
        } finally {
            setBusy(false);
        }
    };

    return (
        <>
            <div className="row">
                <span className="row-label">Status</span>
                <span className={`hook-status ${installed ? "hook-ok" : "hook-missing"}`}>
                    {installed ? "✓ Installed" : "✗ Not installed"}
                </span>
            </div>
            <p className="hook-desc">
                {installed
                    ? "Claude plays a sound after responding. Hook is in ~/.claude/settings.json."
                    : "Add a Stop hook to Claude's settings so a sound plays when Claude finishes."}
            </p>
            <div className="hook-actions">
                {installed ? (
                    <button
                        className="btn-ghost btn-danger"
                        onClick={() => handle(onUninstall)}
                        disabled={busy}
                    >
                        {busy ? "Removing..." : "Uninstall"}
                    </button>
                ) : (
                    <button
                        className="btn-primary"
                        onClick={() => handle(onInstall)}
                        disabled={busy}
                    >
                        {busy ? "Installing..." : "Setup"}
                    </button>
                )}
            </div>
        </>
    );
}

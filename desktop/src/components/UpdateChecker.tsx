import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface DesktopUpdateInfo {
    current_version: string;
    latest_version: string;
    update_available: boolean;
    release_url: string;
    download_url: string | null;
}

type UpdateState =
    | { kind: "idle" }
    | { kind: "loading" }
    | { kind: "error"; message: string }
    | { kind: "result"; info: DesktopUpdateInfo };

export default function UpdateChecker() {
    const [state, setState] = useState<UpdateState>({ kind: "idle" });

    const check = async () => {
        setState({ kind: "loading" });
        try {
            const info = await invoke<DesktopUpdateInfo>("check_desktop_update");
            setState({ kind: "result", info });
        } catch (e) {
            const msg =
                e instanceof Error ? e.message : String(e);
            const friendly =
                msg.includes("rate limit") || msg.includes("403")
                    ? "GitHub rate limited. Try again in a bit."
                    : msg.includes("network") || msg.includes("fetch") || msg.includes("connect")
                      ? "Can't reach GitHub. Check your network."
                      : `Check failed: ${msg}`;
            setState({ kind: "error", message: friendly });
        }
    };

    const href =
        state.kind === "result" && state.info.update_available
            ? (state.info.download_url ?? state.info.release_url)
            : undefined;

    return (
        <div className="update-section">
            <div className="update-row">
                <span className="update-label">🔄 Updates</span>
                <button
                    className="btn-primary btn-sm"
                    onClick={check}
                    disabled={state.kind === "loading"}
                >
                    {state.kind === "loading" ? "Checking…" : "Check"}
                </button>
            </div>

            {state.kind === "loading" && (
                <div className="update-msg update-loading">
                    Asking GitHub…
                </div>
            )}

            {state.kind === "error" && (
                <div className="update-msg update-error">
                    {state.message}
                </div>
            )}

            {state.kind === "result" && (
                <div className="update-msg">
                    {state.info.update_available ? (
                        <>
                            <div className="update-badge">New!</div>
                            <div className="update-versions">
                                v{state.info.current_version} →{" "}
                                <strong>v{state.info.latest_version}</strong>
                            </div>
                            <a
                                className="btn-primary btn-sm update-dl"
                                href={href}
                                target="_blank"
                                rel="noreferrer"
                            >
                                Download
                            </a>
                        </>
                    ) : (
                        <div className="update-uptodate">
                            v{state.info.current_version} is the latest 🎉
                        </div>
                    )}
                </div>
            )}
        </div>
    );
}

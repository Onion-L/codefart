import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";
import ThemePicker from "./components/ThemePicker";
import SoundUpload from "./components/SoundUpload";
import NotificationPrefs from "./components/NotificationPrefs";
import AutostartToggle from "./components/AutostartToggle";

interface ThemeInfo {
    name: string;
    desc: string;
}

interface DesktopState {
    theme: string;
    custom_sound: string | null;
    notification_enabled: boolean;
    notification_title: string;
    notification_body: string;
    hook_installed: boolean;
    autostart: boolean;
    themes: ThemeInfo[];
}

type Tab = "sound" | "hook" | "notify" | "startup";

const TABS: { key: Tab; icon: string; label: string }[] = [
    { key: "sound", icon: "🔊", label: "Sound" },
    { key: "hook", icon: "🪝", label: "Hook" },
    { key: "notify", icon: "🔔", label: "Notify" },
    { key: "startup", icon: "⚡", label: "Startup" },
];

function App() {
    const [activeTab, setActiveTab] = useState<Tab>("sound");
    const [state, setState] = useState<DesktopState | null>(null);
    const [hookInstalling, setHookInstalling] = useState(false);

    const refresh = async () => {
        try {
            const s = await invoke<DesktopState>("get_state");
            setState(s);
        } catch (e) {
            console.error("get_state failed:", e);
        }
    };

    useEffect(() => {
        refresh();
    }, []);

    const after = async (fn: () => Promise<void>) => {
        await fn();
        await refresh();
    };

    const handleSelectTheme = (theme: string) => {
        after(() => invoke("set_theme", { theme }));
    };

    const handlePreview = (theme: string) => {
        invoke("preview_theme", { theme });
    };

    const handleUploadSound = async () => {
        try {
            const { open } = await import("@tauri-apps/plugin-dialog");
            const selected = await open({
                multiple: false,
                filters: [{ name: "Audio", extensions: ["wav", "mp3", "flac", "ogg", "aiff", "m4a"] }],
            });
            if (selected) {
                await after(() => invoke("set_custom_sound", { path: selected }));
            }
        } catch (e) {
            console.error("upload sound failed:", e);
        }
    };

    const handleClearSound = () => {
        after(() => invoke("clear_custom_sound"));
    };

    const handleInstallHook = async () => {
        setHookInstalling(true);
        try {
            await after(() => invoke("install_hook"));
        } finally {
            setHookInstalling(false);
        }
    };

    const handleNotifyChange = (enabled: boolean, title: string, body: string) => {
        after(() => invoke("set_notification_preferences", { enabled, title, body }));
    };

    const handleAutostartToggle = (enabled: boolean) => {
        after(() => invoke("set_autostart", { enabled }));
    };

    if (!state) return null;

    return (
        <>
            <div
                className="drag-region"
                data-tauri-drag-region
                onMouseDown={() => {
                    void getCurrentWindow().startDragging();
                }}
            />

            <div className="tab-bar">
                {TABS.map((tab) => (
                    <button
                        key={tab.key}
                        className={`tab ${activeTab === tab.key ? "tab-active" : ""}`}
                        onClick={() => setActiveTab(tab.key)}
                    >
                        <span className="tab-icon">{tab.icon}</span>
                        <span className="tab-label">{tab.label}</span>
                    </button>
                ))}
            </div>

            <div className="tab-content">
                {activeTab === "sound" && (
                    <div className="section">
                        <ThemePicker
                            themes={state.themes}
                            current={state.theme}
                            onSelect={handleSelectTheme}
                            onPreview={handlePreview}
                        />
                        <SoundUpload
                            customSound={state.custom_sound}
                            onUpload={handleUploadSound}
                            onClear={handleClearSound}
                        />
                    </div>
                )}

                {activeTab === "hook" && (
                    <div className="section">
                        <div className="row">
                            <span className="row-label">Claude Code hook</span>
                            <span className={`hook-status ${state.hook_installed ? "hook-ok" : "hook-missing"}`}>
                                {state.hook_installed ? "✓ Installed" : "✗ Not installed"}
                            </span>
                        </div>
                        <p className="hook-desc">
                            {state.hook_installed
                                ? "Hook is in ~/.claude/settings.json. Claude will play a sound after responding."
                                : "CodeFart needs to add a Stop hook to Claude's settings."}
                        </p>
                        {!state.hook_installed && (
                            <button
                                className="btn-primary"
                                onClick={handleInstallHook}
                                disabled={hookInstalling}
                                style={{ marginTop: 12 }}
                            >
                                {hookInstalling ? "Installing..." : "Setup Hook"}
                            </button>
                        )}
                    </div>
                )}

                {activeTab === "notify" && (
                    <div className="section">
                        <NotificationPrefs
                            enabled={state.notification_enabled}
                            title={state.notification_title}
                            body={state.notification_body}
                            onToggle={(enabled) => handleNotifyChange(enabled, state.notification_title, state.notification_body)}
                            onTitleChange={(title) => handleNotifyChange(state.notification_enabled, title, state.notification_body)}
                            onBodyChange={(body) => handleNotifyChange(state.notification_enabled, state.notification_title, body)}
                        />
                    </div>
                )}

                {activeTab === "startup" && (
                    <div className="section">
                        <AutostartToggle
                            enabled={state.autostart}
                            onToggle={handleAutostartToggle}
                        />
                    </div>
                )}
            </div>
        </>
    );
}

export default App;

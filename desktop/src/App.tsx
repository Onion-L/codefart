import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";
import ThemePicker from "./components/ThemePicker";
import SoundUpload from "./components/SoundUpload";
import NotificationPrefs from "./components/NotificationPrefs";
import AutostartToggle from "./components/AutostartToggle";
import HookSection from "./components/HookSection";

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

type Tab = "sound" | "settings";

const TABS: { key: Tab; icon: string; label: string }[] = [
    { key: "sound", icon: "🔊", label: "Sound" },
    { key: "settings", icon: "🔧", label: "Settings" },
];

function App() {
    const [activeTab, setActiveTab] = useState<Tab>("sound");
    const [state, setState] = useState<DesktopState | null>(null);

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

    const handleInstallHook = () => {
        after(() => invoke("install_hook"));
    };

    const handleUninstallHook = () => {
        after(() => invoke("uninstall_hook"));
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

                {activeTab === "settings" && (
                    <>
                        <div className="section">
                            <div className="setting-title">🪝 Claude Hook</div>
                            <HookSection
                                installed={state.hook_installed}
                                onInstall={handleInstallHook}
                                onUninstall={handleUninstallHook}
                            />
                        </div>

                        <div className="section">
                            <div className="setting-title">🔔 Notification</div>
                            <NotificationPrefs
                                enabled={state.notification_enabled}
                                title={state.notification_title}
                                body={state.notification_body}
                                onToggle={(enabled) => handleNotifyChange(enabled, state.notification_title, state.notification_body)}
                                onTitleChange={(title) => handleNotifyChange(state.notification_enabled, title, state.notification_body)}
                                onBodyChange={(body) => handleNotifyChange(state.notification_enabled, state.notification_title, body)}
                            />
                        </div>

                        <div className="section">
                            <div className="setting-title">⚡ Startup</div>
                            <AutostartToggle
                                enabled={state.autostart}
                                onToggle={handleAutostartToggle}
                            />
                        </div>
                    </>
                )}
            </div>
        </>
    );
}

export default App;

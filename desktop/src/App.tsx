import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";
import ThemePicker from "./components/ThemePicker";
import SoundUpload from "./components/SoundUpload";
import NotificationPrefs from "./components/NotificationPrefs";
import AutostartToggle from "./components/AutostartToggle";

interface Theme {
    name: string;
    desc: string;
}

interface DesktopState {
    theme: string;
    custom_sound: string | null;
    hook_installed: boolean;
    autostart: boolean;
    themes: Theme[];
}

const FALLBACK_THEMES: Theme[] = [
    { name: "classic", desc: "The signature sound" },
    { name: "wet", desc: "A wetter variant" },
    { name: "tiny", desc: "Small & polite" },
    { name: "squeaky", desc: "High-pitched" },
    { name: "thunder", desc: "For long CI runs" },
];

type Tab = "sound" | "notify" | "startup";

const TABS: { key: Tab; icon: string; label: string }[] = [
    { key: "sound", icon: "🔊", label: "Sound" },
    { key: "notify", icon: "🔔", label: "Notify" },
    { key: "startup", icon: "⚡", label: "Startup" },
];

function App() {
    const [activeTab, setActiveTab] = useState<Tab>("sound");
    const [themes, setThemes] = useState<Theme[]>(FALLBACK_THEMES);
    const [theme, setTheme] = useState("classic");
    const [customSound, setCustomSound] = useState<string | null>(null);
    const [notifyEnabled, setNotifyEnabled] = useState(true);
    const [notifyTitle, setNotifyTitle] = useState("Claude");
    const [notifyBody, setNotifyBody] = useState("已完成");
    const [autostart, setAutostart] = useState(false);
    const [hookInstalled, setHookInstalled] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const applyState = (state: DesktopState) => {
        setTheme(state.theme);
        setCustomSound(state.custom_sound);
        setHookInstalled(state.hook_installed);
        setAutostart(state.autostart);
        setThemes(state.themes);
    };

    const runAction = async (action: () => Promise<void>) => {
        setError(null);
        try {
            await action();
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
        }
    };

    useEffect(() => {
        void runAction(async () => {
            applyState(await invoke<DesktopState>("get_state"));
        });
    }, []);

    const handleThemeSelect = (name: string) => {
        void runAction(async () => {
            applyState(await invoke<DesktopState>("set_theme", { theme: name }));
        });
    };

    const handlePreview = (name: string) => {
        void runAction(async () => {
            await invoke("preview_theme", { theme: name });
        });
    };

    const handleUpload = () => {
        void runAction(async () => {
            const selected = await open({
                multiple: false,
                directory: false,
                filters: [
                    {
                        name: "Audio",
                        extensions: ["wav", "mp3", "flac", "ogg", "m4a"],
                    },
                ],
            });

            if (typeof selected !== "string") {
                return;
            }

            applyState(
                await invoke<DesktopState>("set_custom_sound", { path: selected }),
            );
        });
    };

    const handleClearSound = () => {
        void runAction(async () => {
            applyState(await invoke<DesktopState>("clear_custom_sound"));
        });
    };

    const handleInstallHook = () => {
        void runAction(async () => {
            applyState(await invoke<DesktopState>("install_hook"));
        });
    };

    const handleAutostart = (enabled: boolean) => {
        void runAction(async () => {
            applyState(
                await invoke<DesktopState>("set_autostart", { enabled }),
            );
        });
    };

    return (
        <>
            <div
                className="drag-region"
                data-tauri-drag-region
                onMouseDown={() => {
                    void getCurrentWindow().startDragging();
                }}
            />

            {error && <div className="error-banner">{error}</div>}

            {/* Tab bar */}
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

            {/* Content */}
            <div className="tab-content">
                {activeTab === "sound" && (
                    <div className="section">
                        <ThemePicker
                            themes={themes}
                            current={theme}
                            onSelect={handleThemeSelect}
                            onPreview={handlePreview}
                        />
                        <SoundUpload
                            customSound={customSound}
                            onUpload={handleUpload}
                            onClear={handleClearSound}
                        />
                    </div>
                )}

                {activeTab === "notify" && (
                    <div className="section">
                        <div className="row hook-row">
                            <div>
                                <span className="row-label">Claude Code hook</span>
                                <p className="row-help">
                                    {hookInstalled
                                        ? "Installed in ~/.claude/settings.json"
                                        : "Run setup to enable completion alerts"}
                                </p>
                            </div>
                            <button
                                className="btn-primary"
                                onClick={handleInstallHook}
                            >
                                {hookInstalled ? "Installed" : "Setup"}
                            </button>
                        </div>
                        <NotificationPrefs
                            enabled={notifyEnabled}
                            title={notifyTitle}
                            body={notifyBody}
                            onToggle={setNotifyEnabled}
                            onTitleChange={setNotifyTitle}
                            onBodyChange={setNotifyBody}
                        />
                    </div>
                )}

                {activeTab === "startup" && (
                    <div className="section">
                        <AutostartToggle
                            enabled={autostart}
                            onToggle={handleAutostart}
                        />
                    </div>
                )}
            </div>
        </>
    );
}

export default App;

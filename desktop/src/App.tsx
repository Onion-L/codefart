import { useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";
import ThemePicker from "./components/ThemePicker";
import SoundUpload from "./components/SoundUpload";
import NotificationPrefs from "./components/NotificationPrefs";
import AutostartToggle from "./components/AutostartToggle";

const THEMES = [
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
    const [theme, setTheme] = useState("classic");
    const [customSound, setCustomSound] = useState<string | null>(null);
    const [notifyEnabled, setNotifyEnabled] = useState(true);
    const [notifyTitle, setNotifyTitle] = useState("Claude");
    const [notifyBody, setNotifyBody] = useState("已完成");
    const [autostart, setAutostart] = useState(false);

    const handlePreview = (_name: string) => {
        // TODO: invoke("preview_theme", { theme: name })
    };

    const handleUpload = () => {
        // TODO: invoke("upload_sound")
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
                            themes={THEMES}
                            current={theme}
                            onSelect={setTheme}
                            onPreview={handlePreview}
                        />
                        <SoundUpload
                            customSound={customSound}
                            onUpload={handleUpload}
                            onClear={() => setCustomSound(null)}
                        />
                    </div>
                )}

                {activeTab === "notify" && (
                    <div className="section">
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
                            onToggle={setAutostart}
                        />
                    </div>
                )}
            </div>
        </>
    );
}

export default App;

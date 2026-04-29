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

function App() {
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
            {/* Notification */}
            <div className="section">
                <div className="section-title">🔔 Notification</div>
                <NotificationPrefs
                    enabled={notifyEnabled}
                    title={notifyTitle}
                    body={notifyBody}
                    onToggle={setNotifyEnabled}
                    onTitleChange={setNotifyTitle}
                    onBodyChange={setNotifyBody}
                />
            </div>

            {/* Sound */}
            <div className="section">
                <div className="section-title">🔊 Sound</div>
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

            {/* Autostart */}
            <div className="section">
                <div className="section-title">⚡ Startup</div>
                <AutostartToggle
                    enabled={autostart}
                    onToggle={setAutostart}
                />
            </div>
        </>
    );
}

export default App;

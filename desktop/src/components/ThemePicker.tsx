interface Theme {
    name: string;
    desc: string;
}

interface ThemePickerProps {
    themes: Theme[];
    current: string;
    onSelect: (name: string) => void;
    onPreview: (name: string) => void;
}

export default function ThemePicker({
    themes,
    current,
    onSelect,
    onPreview,
}: ThemePickerProps) {
    return (
        <div className="theme-list">
            {themes.map((t) => (
                <div
                    key={t.name}
                    className={`theme-row ${t.name === current ? "selected" : ""}`}
                    onClick={() => onSelect(t.name)}
                >
                    <span className="theme-dot" />
                    <span className="theme-name">{t.name}</span>
                    <span className="theme-desc">{t.desc}</span>
                    <button
                        className="preview-btn"
                        onClick={(e) => {
                            e.stopPropagation();
                            onPreview(t.name);
                        }}
                        title="Preview"
                    >
                        🔊
                    </button>
                </div>
            ))}
        </div>
    );
}

import ToggleSwitch from "./ToggleSwitch";

interface NotificationPrefsProps {
    enabled: boolean;
    title: string;
    body: string;
    onToggle: (enabled: boolean) => void;
    onTitleChange: (title: string) => void;
    onBodyChange: (body: string) => void;
}

export default function NotificationPrefs({
    enabled,
    title,
    body,
    onToggle,
    onTitleChange,
    onBodyChange,
}: NotificationPrefsProps) {
    return (
        <>
            <div className="row">
                <span className="row-label">Notifications</span>
                <ToggleSwitch checked={enabled} onChange={onToggle} />
            </div>
            {enabled && (
                <>
                    <div className="input-row">
                        <label>Title</label>
                        <input
                            className="input"
                            value={title}
                            onChange={(e) => onTitleChange(e.target.value)}
                            placeholder="Claude"
                        />
                    </div>
                    <div className="input-row">
                        <label>Body</label>
                        <input
                            className="input"
                            value={body}
                            onChange={(e) => onBodyChange(e.target.value)}
                            placeholder="已完成"
                        />
                    </div>
                </>
            )}
        </>
    );
}

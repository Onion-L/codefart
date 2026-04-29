import ToggleSwitch from "./ToggleSwitch";

interface AutostartToggleProps {
    enabled: boolean;
    onToggle: (enabled: boolean) => void;
}

export default function AutostartToggle({ enabled, onToggle }: AutostartToggleProps) {
    return (
        <div className="row">
            <span className="row-label">Launch at login</span>
            <ToggleSwitch checked={enabled} onChange={onToggle} />
        </div>
    );
}

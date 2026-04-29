interface ToggleSwitchProps {
    checked: boolean;
    onChange: (checked: boolean) => void;
    disabled?: boolean;
}

export default function ToggleSwitch({ checked, onChange }: ToggleSwitchProps) {
    return (
        <button
            className={`toggle ${checked ? "active" : ""}`}
            onClick={() => onChange(!checked)}
            role="switch"
            aria-checked={checked}
        >
            <span className="toggle-knob" />
        </button>
    );
}

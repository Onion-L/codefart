interface SoundUploadProps {
    customSound: string | null;
    onUpload: () => void;
    onClear: () => void;
}

export default function SoundUpload({
    customSound,
    onUpload,
    onClear,
}: SoundUploadProps) {
    return (
        <div className="upload-row">
            <button className="btn-primary" onClick={onUpload}>
                Upload custom...
            </button>
            {customSound && (
                <>
                    <span className="custom-sound-name" title={customSound}>
                        {customSound.split("/").pop()}
                    </span>
                    <button className="btn-ghost" onClick={onClear}>
                        clear
                    </button>
                </>
            )}
        </div>
    );
}

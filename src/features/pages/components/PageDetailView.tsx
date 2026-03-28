import { EditorContent } from "@tiptap/react";
import type { Editor } from "@tiptap/react";
import type { KeyboardEvent } from "react";

type Props = {
  hasPage: boolean;
  isCreateMode: boolean;
  titleInput: string;
  creating: boolean;
  savedAt: Date | null;
  characterCount: number;
  onTitleChange: (value: string) => void;
  onTitleKeyDown: (e: KeyboardEvent<HTMLInputElement>) => void;
  onBack: () => void;
  onSave: () => void;
  editor: Editor | null;
};

export const PageDetailView = ({
  hasPage,
  isCreateMode,
  titleInput,
  creating,
  savedAt,
  characterCount,
  onTitleChange,
  onTitleKeyDown,
  onBack,
  onSave,
  editor,
}: Props) => {
  const savedAtLabel = savedAt ? `Saved ${savedAt.toLocaleTimeString()}` : "";

  if (!hasPage && !isCreateMode) {
    return (
      <div className="page-detail">
        <div className="page-detail-header">
          <button type="button" className="back-button" onClick={onBack}>
            ← Back
          </button>
        </div>
        <p>Loading…</p>
      </div>
    );
  }

  return (
    <div className="page-detail">
      <div className="page-detail-header">
        <button type="button" className="back-button" onClick={onBack}>
          ← Back
        </button>
      </div>
      <div className="page-detail-content">
        <input
          type="text"
          className="page-detail-title-input"
          placeholder={isCreateMode ? "Page title" : undefined}
          value={titleInput}
          onChange={(e) => onTitleChange(e.target.value)}
          onKeyDown={onTitleKeyDown}
          autoFocus={isCreateMode}
        />
        <EditorContent editor={editor} />
        <div className="editor-footer">
          <span className="saved-at">{savedAtLabel}</span>
          <div className="editor-footer-right">
            <span className="character-count">{characterCount} characters</span>
            <button
              type="button"
              className="save-button"
              onClick={onSave}
              disabled={isCreateMode && (!titleInput.trim() || creating)}
            >
              {isCreateMode && creating ? "Creating..." : "Save"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

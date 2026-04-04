import { EditorContent } from "@tiptap/react";
import type { Editor } from "@tiptap/react";
import type { CSSProperties } from "react";
import type { KeyboardEvent } from "react";

type TableToolbarPosition = {
  top: number;
  left: number;
};

type Props = {
  hasPage: boolean;
  isCreateMode: boolean;
  isTableActive: boolean;
  tableToolbarPosition: TableToolbarPosition | null;
  titleInput: string;
  creating: boolean;
  savedAt: Date | null;
  characterCount: number;
  onTitleChange: (value: string) => void;
  onTitleKeyDown: (e: KeyboardEvent<HTMLInputElement>) => void;
  onBack: () => void;
  onAddColumn: () => void;
  onDeleteColumn: () => void;
  onSave: () => void;
  editor: Editor | null;
};

export const PageDetailView = ({
  hasPage,
  isCreateMode,
  isTableActive,
  tableToolbarPosition,
  titleInput,
  creating,
  savedAt,
  characterCount,
  onTitleChange,
  onTitleKeyDown,
  onBack,
  onAddColumn,
  onDeleteColumn,
  onSave,
  editor,
}: Props) => {
  const savedAtLabel = savedAt ? `Saved ${savedAt.toLocaleTimeString()}` : "";
  const tableToolbarStyle: CSSProperties | undefined = tableToolbarPosition
    ? {
      top: tableToolbarPosition.top,
      left: tableToolbarPosition.left,
    }
    : undefined;

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
        {isTableActive && tableToolbarPosition && (
          <div className="table-floating-toolbar" style={tableToolbarStyle}>
            <button
              type="button"
              className="table-toolbar-button"
              onClick={onAddColumn}
            >
              + Column
            </button>
            <button
              type="button"
              className="table-toolbar-button"
              onClick={onDeleteColumn}
            >
              - Column
            </button>
          </div>
        )}
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

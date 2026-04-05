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
  deleting: boolean;
  isDeleteModalOpen: boolean;
  savedAt: Date | null;
  characterCount: number;
  onTitleChange: (value: string) => void;
  onTitleKeyDown: (e: KeyboardEvent<HTMLInputElement>) => void;
  onBack: () => void;
  onAddColumn: () => void;
  onDeleteColumn: () => void;
  onSave: () => void;
  onDelete: () => void;
  onCancelDelete: () => void;
  onConfirmDelete: () => void;
  editor: Editor | null;
};

export const PageDetailView = ({
  hasPage,
  isCreateMode,
  isTableActive,
  tableToolbarPosition,
  titleInput,
  creating,
  deleting,
  isDeleteModalOpen,
  savedAt,
  characterCount,
  onTitleChange,
  onTitleKeyDown,
  onBack,
  onAddColumn,
  onDeleteColumn,
  onSave,
  onDelete,
  onCancelDelete,
  onConfirmDelete,
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
    <>
      <div className="page-detail">
        <div className="page-detail-header">
          <button type="button" className="back-button" onClick={onBack}>
            ← Back
          </button>
          {!isCreateMode && hasPage && (
            <button
              type="button"
              className="delete-button"
              onClick={onDelete}
              disabled={deleting}
            >
              {deleting ? "Deleting..." : "Delete"}
            </button>
          )}
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
              <span className="character-count">
                {characterCount} characters
              </span>
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
      {isDeleteModalOpen && (
        <div
          className="modal-overlay"
          role="presentation"
          onClick={onCancelDelete}
        >
          <div
            className="confirm-modal"
            role="dialog"
            aria-modal="true"
            aria-labelledby="delete-modal-title"
            onClick={(event) => event.stopPropagation()}
          >
            <h2 id="delete-modal-title" className="confirm-modal-title">
              Delete page?
            </h2>
            <p className="confirm-modal-text">
              This page will be permanently deleted.
            </p>
            <div className="confirm-modal-actions">
              <button
                type="button"
                className="modal-button"
                onClick={onCancelDelete}
                disabled={deleting}
              >
                Cancel
              </button>
              <button
                type="button"
                className="modal-button modal-button-danger"
                onClick={onConfirmDelete}
                disabled={deleting}
              >
                {deleting ? "Deleting..." : "Delete"}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

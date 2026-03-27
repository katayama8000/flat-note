import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Page } from "../types.ts";

export function Header() {
  const navigate = useNavigate();
  const [showCreate, setShowCreate] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [creating, setCreating] = useState(false);

  const handleCreate = async () => {
    const title = newTitle.trim();
    if (!title) return;
    setCreating(true);
    try {
      const page = await invoke<Page>("create_page", { title });
      setShowCreate(false);
      setNewTitle("");
      navigate({ to: "/pages/$pageId", params: { pageId: page.id } });
    } finally {
      setCreating(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") handleCreate();
    if (e.key === "Escape") {
      setShowCreate(false);
      setNewTitle("");
    }
  };

  return (
    <>
      <header className="app-header">
        <div className="app-header-left" onClick={() => navigate({ to: "/" })}>
          <div className="app-header-icon">f</div>
          <span className="app-header-title">flat note</span>
        </div>
        <div className="app-header-center">
          <input
            className="app-header-search"
            type="text"
            placeholder="Search..."
            readOnly
          />
        </div>
        <div className="app-header-right">
          <button
            className="app-header-create-btn"
            aria-label="New page"
            onClick={() => setShowCreate(true)}
            type="button"
          >
            +
          </button>
        </div>
      </header>

      {showCreate && (
        <div
          className="create-page-overlay"
          onClick={() => {
            setShowCreate(false);
            setNewTitle("");
          }}
        >
          <div
            className="create-page-dialog"
            onClick={(e) => e.stopPropagation()}
          >
            <p className="create-page-label">New page</p>
            <input
              className="create-page-input"
              type="text"
              placeholder="Page title"
              value={newTitle}
              onChange={(e) => setNewTitle(e.target.value)}
              onKeyDown={handleKeyDown}
              autoFocus
            />
            <div className="create-page-actions">
              <button
                className="create-page-cancel"
                onClick={() => {
                  setShowCreate(false);
                  setNewTitle("");
                }}
                type="button"
              >
                Cancel
              </button>
              <button
                className="create-page-confirm"
                onClick={handleCreate}
                disabled={!newTitle.trim() || creating}
                type="button"
              >
                {creating ? "Creating..." : "Create"}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

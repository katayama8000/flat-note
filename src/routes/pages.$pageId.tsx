import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import CharacterCount from "@tiptap/extension-character-count";
import Youtube from "@tiptap/extension-youtube";
import CodeBlockLowlight from "@tiptap/extension-code-block-lowlight";
import { all, createLowlight } from "lowlight";
import type { Page } from "../types.ts";
import "../App.css";

export const Route = createFileRoute("/pages/$pageId")({
  component: PageDetail,
});

function PageDetail() {
  const { pageId } = Route.useParams();
  const isCreateMode = pageId === "new";
  const navigate = useNavigate();
  const [page, setPage] = useState<Page | null>(null);
  const [titleInput, setTitleInput] = useState("");
  const [creating, setCreating] = useState(false);
  const [savedAt, setSavedAt] = useState<Date | null>(null);
  const autoSaveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const lastTitleEnterAt = useRef(0);
  const hasSaved = useRef(false);

  const editor = useEditor({
    extensions: [
      StarterKit.configure({ codeBlock: false }),
      CodeBlockLowlight.configure({ lowlight: createLowlight(all) }),
      TaskList,
      TaskItem.configure({ nested: true }),
      CharacterCount,
      Youtube.configure({ width: 640, height: 360, autoplay: false }),
    ],
    content: "",
    editorProps: {
      attributes: {
        class: "tiptap-editor",
      },
    },
  });

  const handleTitleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.nativeEvent.isComposing) return;
      if (e.key !== "Enter") return;
      e.preventDefault();

      const now = Date.now();
      const isSecondEnter = now - lastTitleEnterAt.current <= 700;
      lastTitleEnterAt.current = now;
      if (isSecondEnter) {
        editor?.commands.focus("start");
        lastTitleEnterAt.current = 0;
      }
    },
    [editor],
  );

  useEffect(() => {
    if (!editor) return;
    hasSaved.current = false;
    if (isCreateMode) {
      setPage(null);
      setTitleInput("");
      editor.commands.setContent("");
      return;
    }

    invoke<Page | null>("get_page", { id: pageId }).then((p) => {
      setPage(p);
      if (!p) return;
      setTitleInput(p.title);
      const content = p.description.startsWith("<")
        ? p.description
        : `<p>${p.description}</p>`;
      editor.commands.setContent(content);
    });
  }, [pageId, isCreateMode, editor]);

  const handleSave = useCallback(async () => {
    if (!editor) return;

    if (isCreateMode) {
      const title = titleInput.trim();
      if (!title || creating) return;

      setCreating(true);
      try {
        const created = await invoke<Page>("create_page", { title });
        await invoke("update_page", {
          id: created.id,
          description: editor.getHTML(),
        });
        setSavedAt(new Date());
        setPage(created);
        navigate({ to: "/pages/$pageId", params: { pageId: created.id } });
      } finally {
        setCreating(false);
      }
      return;
    }

    if (!page) return;

    const trimmedTitle = titleInput.trim();
    if (hasSaved.current) {
      if (trimmedTitle && trimmedTitle !== page.title) {
        await invoke("update_title_direct", {
          id: page.id,
          title: trimmedTitle,
        });
        setPage({ ...page, title: trimmedTitle });
      }
      await invoke("update_page_direct", {
        id: page.id,
        description: editor.getHTML(),
      });
    } else {
      if (trimmedTitle && trimmedTitle !== page.title) {
        await invoke("update_title", { id: page.id, title: trimmedTitle });
        setPage({ ...page, title: trimmedTitle });
      }
      await invoke("update_page", {
        id: page.id,
        description: editor.getHTML(),
      });
      hasSaved.current = true;
    }

    setSavedAt(new Date());
  }, [editor, page, isCreateMode, titleInput, creating, navigate]);

  // 1.5秒後に自動保存
  useEffect(() => {
    if (!editor) return;
    const onUpdate = () => {
      if (autoSaveTimer.current) clearTimeout(autoSaveTimer.current);
      autoSaveTimer.current = setTimeout(() => {
        handleSave();
      }, 1500);
    };
    editor.on("update", onUpdate);
    return () => {
      editor.off("update", onUpdate);
      if (autoSaveTimer.current) clearTimeout(autoSaveTimer.current);
    };
  }, [editor, handleSave]);

  // ウィンドウを閉じる時に保存
  useEffect(() => {
    const win = getCurrentWindow();
    let unlisten: (() => void) | undefined;
    win.onCloseRequested(async (event) => {
      event.preventDefault();
      await handleSave();
      await win.destroy();
    }).then((fn) => {
      unlisten = fn;
    });
    return () => unlisten?.();
  }, [handleSave]);

  const handleBack = useCallback(async () => {
    if (autoSaveTimer.current) {
      clearTimeout(autoSaveTimer.current);
      autoSaveTimer.current = null;
    }

    if (isCreateMode && !titleInput.trim()) {
      navigate({ to: "/" });
      return;
    }

    await handleSave();
    navigate({ to: "/" });
  }, [handleSave, navigate, isCreateMode, titleInput]);

  // Cmd+S / Ctrl+S で保存
  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "s") {
        e.preventDefault();
        handleSave();
      }
    };
    globalThis.addEventListener("keydown", onKeyDown);
    return () => globalThis.removeEventListener("keydown", onKeyDown);
  }, [handleSave]);

  return (
    <div className="page-detail">
      <div className="page-detail-header">
        <button
          type="button"
          className="back-button"
          onClick={handleBack}
        >
          ← Back
        </button>
      </div>
      {page
        ? (
          <div className="page-detail-content">
            <input
              type="text"
              className="page-detail-title-input"
              value={titleInput}
              onChange={(e) => setTitleInput(e.target.value)}
              onKeyDown={handleTitleKeyDown}
            />
            <EditorContent editor={editor} />
            <div className="editor-footer">
              <span className="saved-at">
                {savedAt ? `Saved ${savedAt.toLocaleTimeString()}` : ""}
              </span>
              <div className="editor-footer-right">
                <span className="character-count">
                  {editor?.storage.characterCount.characters()} characters
                </span>
                <button
                  type="button"
                  className="save-button"
                  onClick={handleSave}
                >
                  Save
                </button>
              </div>
            </div>
          </div>
        )
        : isCreateMode
        ? (
          <div className="page-detail-content">
            <input
              type="text"
              className="page-detail-title-input"
              placeholder="Page title"
              value={titleInput}
              onChange={(e) => setTitleInput(e.target.value)}
              onKeyDown={handleTitleKeyDown}
              autoFocus
            />
            <EditorContent editor={editor} />
            <div className="editor-footer">
              <span className="saved-at">
                {savedAt ? `Saved ${savedAt.toLocaleTimeString()}` : ""}
              </span>
              <div className="editor-footer-right">
                <span className="character-count">
                  {editor?.storage.characterCount.characters()} characters
                </span>
                <button
                  type="button"
                  className="save-button"
                  onClick={handleSave}
                  disabled={!titleInput.trim() || creating}
                >
                  {creating ? "Creating..." : "Save"}
                </button>
              </div>
            </div>
          </div>
        )
        : <p>Loading…</p>}
    </div>
  );
}

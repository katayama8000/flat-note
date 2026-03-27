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
  const navigate = useNavigate();
  const [page, setPage] = useState<Page | null>(null);
  const [savedAt, setSavedAt] = useState<Date | null>(null);
  const autoSaveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

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

  useEffect(() => {
    invoke<Page>("get_page", { id: pageId }).then((p) => {
      setPage(p);
      if (editor && p) {
        // Plain text を段落として扱う
        const content = p.description.startsWith("<")
          ? p.description
          : `<p>${p.description}</p>`;
        editor.commands.setContent(content);
      }
    });
  }, [pageId, editor]);

  const handleSave = useCallback(async () => {
    if (!editor || !page) return;
    await invoke("update_page", { id: page.id, description: editor.getHTML() });
    setSavedAt(new Date());
  }, [editor, page]);

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
    await handleSave();
    navigate({ to: "/" });
  }, [handleSave, navigate]);

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
            <h1 className="page-detail-title">{page.title}</h1>
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
        : <p>Loading…</p>}
    </div>
  );
}

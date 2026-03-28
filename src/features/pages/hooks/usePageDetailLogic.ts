import { useCallback, useEffect, useRef, useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import CharacterCount from "@tiptap/extension-character-count";
import Youtube from "@tiptap/extension-youtube";
import CodeBlockLowlight from "@tiptap/extension-code-block-lowlight";
import { all, createLowlight } from "lowlight";
import {
  createPage,
  getPage,
  updatePage,
  updatePageDirect,
  updateTitle,
  updateTitleDirect,
} from "../api/pageApi.ts";
import type { Page } from "../types/page.ts";

type Props = {
  pageId: string;
};

export const usePageDetailLogic = ({ pageId }: Props) => {
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

  const handleSave = useCallback(async () => {
    if (!editor) return;

    if (isCreateMode) {
      const title = titleInput.trim();
      if (!title || creating) return;

      setCreating(true);
      try {
        const created = await createPage(title);
        await updatePage(created.id, editor.getHTML());
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
        await updateTitleDirect(page.id, trimmedTitle);
        setPage({ ...page, title: trimmedTitle });
      }
      await updatePageDirect(page.id, editor.getHTML());
    } else {
      if (trimmedTitle && trimmedTitle !== page.title) {
        await updateTitle(page.id, trimmedTitle);
        setPage({ ...page, title: trimmedTitle });
      }
      await updatePage(page.id, editor.getHTML());
      hasSaved.current = true;
    }

    setSavedAt(new Date());
  }, [editor, page, isCreateMode, titleInput, creating, navigate]);

  useEffect(() => {
    if (!editor) return;
    hasSaved.current = false;
    if (isCreateMode) {
      setPage(null);
      setTitleInput("");
      editor.commands.setContent("");
      return;
    }

    getPage(pageId).then((fetchedPage) => {
      setPage(fetchedPage);
      if (!fetchedPage) return;
      setTitleInput(fetchedPage.title);
      const content = fetchedPage.description.startsWith("<")
        ? fetchedPage.description
        : `<p>${fetchedPage.description}</p>`;
      editor.commands.setContent(content);
    });
  }, [pageId, isCreateMode, editor]);

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

  return {
    page,
    editor,
    isCreateMode,
    titleInput,
    creating,
    savedAt,
    setTitleInput,
    handleTitleKeyDown,
    handleSave,
    handleBack,
  };
};

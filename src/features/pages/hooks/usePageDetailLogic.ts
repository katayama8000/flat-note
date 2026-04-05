import { useCallback, useEffect, useRef, useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEditor } from "@tiptap/react";
import type { Editor as TiptapEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import CharacterCount from "@tiptap/extension-character-count";
import Youtube from "@tiptap/extension-youtube";
import CodeBlockLowlight from "@tiptap/extension-code-block-lowlight";
import { Table } from "@tiptap/extension-table";
import { TableRow } from "@tiptap/extension-table-row";
import { TableHeader } from "@tiptap/extension-table-header";
import { TableCell } from "@tiptap/extension-table-cell";
import { Markdown } from "@tiptap/markdown";
import { common, createLowlight } from "lowlight";
import {
  createPage,
  deletePage,
  getPage,
  updatePage,
  updatePageDirect,
  updateTitle,
  updateTitleDirect,
} from "../api/pageApi.ts";
import type { Page } from "../types/page.ts";

type TableToolbarPosition = {
  top: number;
  left: number;
};

type Props = {
  pageId: string;
};

type MarkdownNode = {
  type?: string;
  attrs?: Record<string, unknown>;
  content?: MarkdownNode[];
};

type MarkdownHelpers = {
  renderChildren: (
    node: MarkdownNode | MarkdownNode[],
    separator?: string,
  ) => string;
};

const TABLE_CELL_SEPARATOR = "\u001F";

const hasMarkdownTable = (markdown: string): boolean => {
  return /(?:^|\n)\|.+\|\n\|(?:\s*[:\-]{3,}:?\s*\|)+\n(?:\|.*\|\n?)*/m
    .test(markdown);
};

const normalizeStoredMarkdown = (markdown: string): string => {
  return markdown.split(TABLE_CELL_SEPARATOR).join("<br>");
};

const normalizeTableCellText = (text: string): string => {
  return text
    .split(TABLE_CELL_SEPARATOR).join("<br>")
    .replace(/\\\n/g, "<br>")
    .replace(/\n+/g, "<br>")
    .replace(/\s*<br>\s*/g, "<br>")
    .replace(/\|/g, "\\|")
    .trim();
};

const getTableAlignmentMarker = (align: unknown, width: number): string => {
  const dashCount = Math.max(3, width);

  if (align === "left") {
    return `:${"-".repeat(dashCount)}`;
  }

  if (align === "right") {
    return `${"-".repeat(dashCount)}:`;
  }

  if (align === "center") {
    return `:${"-".repeat(dashCount)}:`;
  }

  return "-".repeat(dashCount);
};

const renderTableToMarkdownWithBreaks = (
  node: MarkdownNode,
  h: MarkdownHelpers,
): string => {
  if (!node.content || node.content.length === 0) {
    return "";
  }

  const rows = node.content.map((rowNode: MarkdownNode) => {
    const cells = rowNode.content ?? [];

    return cells.map((cellNode: MarkdownNode) => {
      const blocks = cellNode.content ?? [];
      const raw = blocks.length > 0
        ? blocks.map((block: MarkdownNode) => h.renderChildren(block)).join(
          "<br>",
        )
        : "";

      return {
        text: normalizeTableCellText(raw),
        isHeader: cellNode.type === "tableHeader",
        align: cellNode.attrs?.align ?? null,
      };
    });
  });

  const columnCount = rows.reduce(
    (max: number, row) => Math.max(max, row.length),
    0,
  );
  if (columnCount === 0) {
    return "";
  }

  const columnWidths = new Array<number>(columnCount).fill(3);
  rows.forEach((row) => {
    for (let index = 0; index < columnCount; index += 1) {
      const text = row[index]?.text ?? "";
      columnWidths[index] = Math.max(columnWidths[index], text.length, 3);
    }
  });

  const hasHeader =
    rows[0]?.some((cell: { isHeader: boolean }) => cell.isHeader) ?? false;
  const columnAlignments = new Array<unknown>(columnCount).fill(null);
  rows.forEach((row) => {
    for (let index = 0; index < columnCount; index += 1) {
      if (!columnAlignments[index] && row[index]?.align) {
        columnAlignments[index] = row[index].align;
      }
    }
  });

  const pad = (text: string, width: number) => {
    return text + " ".repeat(Math.max(0, width - text.length));
  };

  const headerTexts = new Array(columnCount)
    .fill("")
    .map((_, index) => (hasHeader ? rows[0]?.[index]?.text ?? "" : ""));

  let markdown = "\n";
  markdown += `| ${
    headerTexts.map((text, index) => pad(text, columnWidths[index])).join(" | ")
  } |\n`;
  markdown += `| ${
    columnWidths.map((width, index) =>
      getTableAlignmentMarker(columnAlignments[index], width)
    ).join(" | ")
  } |\n`;

  const bodyRows = hasHeader ? rows.slice(1) : rows;
  bodyRows.forEach((row) => {
    const rowText = new Array(columnCount)
      .fill("")
      .map((_, index) => pad(row[index]?.text ?? "", columnWidths[index]));
    markdown += `| ${rowText.join(" | ")} |\n`;
  });

  return markdown;
};

const TableWithPreservedBreaks = Table.extend({
  renderMarkdown(node, h) {
    return renderTableToMarkdownWithBreaks(
      node as MarkdownNode,
      h as MarkdownHelpers,
    );
  },
});

export const usePageDetailLogic = ({ pageId }: Props) => {
  const isCreateMode = pageId === "new";
  const navigate = useNavigate();
  const [page, setPage] = useState<Page | null>(null);
  const [titleInput, setTitleInput] = useState("");
  const [creating, setCreating] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [savedAt, setSavedAt] = useState<Date | null>(null);
  const [isTableActive, setIsTableActive] = useState(false);
  const [tableToolbarPosition, setTableToolbarPosition] = useState<
    TableToolbarPosition | null
  >(null);
  const autoSaveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const lastTitleEnterAt = useRef(0);
  const hasSaved = useRef(false);
  const editorRef = useRef<TiptapEditor | null>(null);

  // Initialize Tiptap editor with Markdown extension
  const editor = useEditor({
    extensions: [
      StarterKit.configure({ codeBlock: false }),
      CodeBlockLowlight.configure({ lowlight: createLowlight(common) }),
      TaskList,
      TaskItem.configure({ nested: true }),
      CharacterCount,
      Youtube.configure({ width: 640, height: 360, autoplay: false }),
      TableWithPreservedBreaks.configure({ resizable: true }),
      TableRow,
      TableHeader,
      TableCell,
      Markdown, // Enable Markdown parsing/serialization
    ],
    content: "",
    contentType: "markdown", // Treat initial content as Markdown
    editorProps: {
      attributes: {
        class: "tiptap-editor",
      },
      handleKeyDown: (_view, event): boolean => {
        if (event.isComposing || event.key !== "Enter") {
          return false;
        }

        const currentEditor = editorRef.current;
        if (!currentEditor) {
          return false;
        }

        const inTableCell = currentEditor.isActive("tableCell") ||
          currentEditor.isActive("tableHeader");
        if (inTableCell) {
          event.preventDefault();
          return currentEditor.chain().focus().setHardBreak().run();
        }

        const { $from } = currentEditor.state.selection;
        const parent = $from.parent;
        if (!parent.isTextblock) {
          return false;
        }

        const commandText = parent.textContent.trim().toLowerCase();
        if (commandText !== "/table") {
          return false;
        }

        const from = $from.start();
        const to = $from.end();

        event.preventDefault();
        currentEditor
          .chain()
          .focus()
          .deleteRange({ from, to })
          .insertTable({ rows: 3, cols: 3, withHeaderRow: true })
          .run();

        return true;
      },
      handlePaste: (_view, event): boolean => {
        const pastedText = event.clipboardData?.getData("text/plain");
        if (!pastedText) {
          return false;
        }

        if (!hasMarkdownTable(pastedText)) {
          return false;
        }

        event.preventDefault();
        return editorRef.current?.commands.insertContent(pastedText, {
          contentType: "markdown",
        }) ?? false;
      },
    },
  });

  useEffect(() => {
    editorRef.current = editor;
  }, [editor]);

  useEffect(() => {
    if (!editor) return;

    const syncTableSelectionState = () => {
      const active = editor.isActive("tableHeader");
      setIsTableActive(active);

      if (!active) {
        setTableToolbarPosition(null);
        return;
      }

      const coords = editor.view.coordsAtPos(editor.state.selection.from);
      setTableToolbarPosition({
        top: coords.top - 44,
        left: coords.left,
      });
    };

    syncTableSelectionState();
    editor.on("selectionUpdate", syncTableSelectionState);
    editor.on("focus", syncTableSelectionState);
    editor.on("blur", syncTableSelectionState);

    return () => {
      editor.off("selectionUpdate", syncTableSelectionState);
      editor.off("focus", syncTableSelectionState);
      editor.off("blur", syncTableSelectionState);
    };
  }, [editor]);

  useEffect(() => {
    if (!editor || !isTableActive) return;

    const onWindowChange = () => {
      if (!editor.isActive("tableHeader")) return;
      const coords = editor.view.coordsAtPos(editor.state.selection.from);
      setTableToolbarPosition({
        top: coords.top - 44,
        left: coords.left,
      });
    };

    globalThis.addEventListener("scroll", onWindowChange, true);
    globalThis.addEventListener("resize", onWindowChange);
    return () => {
      globalThis.removeEventListener("scroll", onWindowChange, true);
      globalThis.removeEventListener("resize", onWindowChange);
    };
  }, [editor, isTableActive]);

  const handleAddColumn = useCallback(() => {
    if (!editor) return;
    editor.chain().focus().addColumnAfter().run();
    setIsTableActive(editor.isActive("tableHeader"));
  }, [editor]);

  const handleDeleteColumn = useCallback(() => {
    if (!editor) return;
    editor.chain().focus().deleteColumn().run();
    setIsTableActive(editor.isActive("tableHeader"));
  }, [editor]);

  // Handle Enter key in title input (double-Enter focuses editor)
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

  // Save page content and title
  const handleSave = useCallback(async () => {
    if (!editor) return;

    if (isCreateMode) {
      const title = titleInput.trim();
      if (!title || creating) return;

      setCreating(true);
      try {
        const created = await createPage(title);
        // Get Markdown content and save it (not JSON)
        const markdown = editor.getMarkdown();
        await updatePage(created.id, markdown);
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
    // Get Markdown content
    const markdown = editor.getMarkdown();

    if (hasSaved.current) {
      if (trimmedTitle && trimmedTitle !== page.title) {
        await updateTitleDirect(page.id, trimmedTitle);
        setPage({ ...page, title: trimmedTitle });
      }
      // Save Markdown, not JSON
      await updatePageDirect(page.id, markdown);
    } else {
      if (trimmedTitle && trimmedTitle !== page.title) {
        await updateTitle(page.id, trimmedTitle);
        setPage({ ...page, title: trimmedTitle });
      }
      // Save Markdown, not JSON
      await updatePage(page.id, markdown);
      hasSaved.current = true;
    }

    setSavedAt(new Date());
  }, [editor, page, isCreateMode, titleInput, creating, navigate]);

  // Load page data and content when pageId changes
  useEffect(() => {
    if (!editor) return;
    // Reset saved flag on page change
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

      // Treat description as Markdown and set it with Markdown content type
      editor.commands.setContent(
        normalizeStoredMarkdown(fetchedPage.description),
        {
          contentType: "markdown",
        },
      );
    });
  }, [pageId, isCreateMode, editor]);

  // Auto-save on editor update (debounced)
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

  // Keep a ref to the current handleSave to avoid re-registering listeners
  const handleSaveRef = useRef(handleSave);
  useEffect(() => {
    handleSaveRef.current = handleSave;
  }, [handleSave]);

  // Save on window close (register once)
  useEffect(() => {
    let unlistenFn: (() => void) | null = null;

    // Register the close handler
    getCurrentWindow().onCloseRequested((event) => {
      event.preventDefault();

      // Create a timeout for save (1 second max to avoid hanging)
      const savePromise = handleSaveRef.current().catch((err) => {
        console.warn("Could not save on close:", err);
        // Continue anyway, as auto-save should have already saved
      });

      const timeoutPromise = new Promise((resolve) => {
        setTimeout(resolve, 1000);
      });

      // Race between save and timeout
      Promise.race([savePromise, timeoutPromise]).then(() => {
        getCurrentWindow().destroy().catch((err) => {
          console.error("Error destroying window:", err);
        });
      });
    }).then((unlisten) => {
      unlistenFn = unlisten;
    }).catch((err) => {
      console.error("Error registering close handler:", err);
    });

    return () => {
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, []);

  // Navigate back with save
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

  const handleDelete = useCallback(() => {
    if (isCreateMode || !page || deleting) {
      return;
    }

    setIsDeleteModalOpen(true);
  }, [deleting, isCreateMode, page]);

  const handleCancelDelete = useCallback(() => {
    if (deleting) {
      return;
    }

    setIsDeleteModalOpen(false);
  }, [deleting]);

  const handleConfirmDelete = useCallback(async () => {
    if (isCreateMode || !page || deleting) {
      return;
    }

    if (autoSaveTimer.current) {
      clearTimeout(autoSaveTimer.current);
      autoSaveTimer.current = null;
    }

    setDeleting(true);
    try {
      await deletePage(page.id);
      setIsDeleteModalOpen(false);
      navigate({ to: "/" });
    } finally {
      setDeleting(false);
    }
  }, [deleting, isCreateMode, navigate, page]);

  // Global save shortcut (Ctrl/Cmd + S) - register once using ref
  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "s") {
        e.preventDefault();
        handleSaveRef.current();
      }
    };
    globalThis.addEventListener("keydown", onKeyDown);
    return () => globalThis.removeEventListener("keydown", onKeyDown);
  }, []);

  return {
    page,
    editor,
    isCreateMode,
    isTableActive,
    tableToolbarPosition,
    titleInput,
    creating,
    deleting,
    isDeleteModalOpen,
    savedAt,
    setTitleInput,
    handleTitleKeyDown,
    handleAddColumn,
    handleDeleteColumn,
    handleSave,
    handleBack,
    handleDelete,
    handleCancelDelete,
    handleConfirmDelete,
  };
};

import { PageDetailView } from "../features/pages/components/PageDetailView.tsx";
import { usePageDetailLogic } from "../features/pages/hooks/usePageDetailLogic.ts";
import "../App.css";

type Props = {
  pageId: string;
};

export const PageDetailPage = ({ pageId }: Props) => {
  const {
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
  } = usePageDetailLogic({ pageId });

  return (
    <PageDetailView
      hasPage={Boolean(page)}
      isCreateMode={isCreateMode}
      isTableActive={isTableActive}
      tableToolbarPosition={tableToolbarPosition}
      titleInput={titleInput}
      creating={creating}
      deleting={deleting}
      isDeleteModalOpen={isDeleteModalOpen}
      savedAt={savedAt}
      characterCount={editor?.storage.characterCount.characters() ?? 0}
      onTitleChange={setTitleInput}
      onTitleKeyDown={handleTitleKeyDown}
      onBack={handleBack}
      onAddColumn={handleAddColumn}
      onDeleteColumn={handleDeleteColumn}
      onSave={handleSave}
      onDelete={handleDelete}
      onCancelDelete={handleCancelDelete}
      onConfirmDelete={handleConfirmDelete}
      editor={editor}
    />
  );
};

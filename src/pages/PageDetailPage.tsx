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
    titleInput,
    creating,
    savedAt,
    setTitleInput,
    handleTitleKeyDown,
    handleSave,
    handleBack,
    relatedPages,
  } = usePageDetailLogic({ pageId });

  return (
    <PageDetailView
      hasPage={Boolean(page)}
      isCreateMode={isCreateMode}
      titleInput={titleInput}
      creating={creating}
      savedAt={savedAt}
      characterCount={editor?.storage.characterCount.characters() ?? 0}
      onTitleChange={setTitleInput}
      onTitleKeyDown={handleTitleKeyDown}
      onBack={handleBack}
      onSave={handleSave}
      editor={editor}
      relatedPages={relatedPages}
    />
  );
};

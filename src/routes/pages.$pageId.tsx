import { createFileRoute } from "@tanstack/react-router";
import PageDetailPage from "../pages/PageDetailPage.tsx";

export const Route = createFileRoute("/pages/$pageId")({
  component: PageDetail,
});

function PageDetail() {
  const { pageId } = Route.useParams();

  return <PageDetailPage pageId={pageId} />;
}

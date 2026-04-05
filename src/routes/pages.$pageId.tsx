import { createFileRoute } from "@tanstack/react-router";
import { lazy, Suspense } from "react";

const LazyPageDetailPage = lazy(() =>
  import("../pages/PageDetailPage.tsx").then((module) => ({
    default: module.PageDetailPage,
  }))
);

export const Route = createFileRoute("/pages/$pageId")({
  component: PageDetail,
});

function PageDetail() {
  const { pageId } = Route.useParams();

  return (
    <Suspense fallback={<div className="page-detail">Loading...</div>}>
      <LazyPageDetailPage pageId={pageId} />
    </Suspense>
  );
}

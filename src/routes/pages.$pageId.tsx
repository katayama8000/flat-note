import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Page } from "../types.ts";

export const Route = createFileRoute("/pages/$pageId")({
  component: PageDetail,
});

function PageDetail() {
  const { pageId } = Route.useParams();
  const navigate = useNavigate();
  const [page, setPage] = useState<Page | null>(null);

  useEffect(() => {
    invoke<Page>("get_page", { id: pageId }).then(setPage);
  }, [pageId]);

  return (
    <div className="page-detail">
      <button
        type="button"
        className="back-button"
        onClick={() => navigate({ to: "/" })}
      >
        ← Back
      </button>
      {page
        ? (
          <div className="page-detail-content">
            <h1 className="page-detail-title">{page.title}</h1>
            <p className="page-detail-description">{page.description}</p>
          </div>
        )
        : <p>Loading...</p>}
    </div>
  );
}

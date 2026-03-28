import { useNavigate } from "@tanstack/react-router";
import type { Page } from "../types.ts";

type Props = {
  page: Page;
};

const stripHtml = (html: string): string => {
  const doc = new DOMParser().parseFromString(html, "text/html");
  return doc.body.textContent ?? "";
};

const extractYoutubeId = (html: string): string | null => {
  const doc = new DOMParser().parseFromString(html, "text/html");
  const iframe = doc.querySelector('iframe[src*="youtube.com/embed"]');
  if (!iframe) return null;
  const src = iframe.getAttribute("src") ?? "";
  const match = src.match(/youtube\.com\/embed\/([^?&]+)/);
  return match ? match[1] : null;
};

export function PageCard({ page }: Props) {
  const navigate = useNavigate();
  const youtubeId = extractYoutubeId(page.description);

  return (
    <div
      className={`page-card${youtubeId ? " page-card--youtube" : ""}`}
      onClick={() =>
        navigate({ to: "/pages/$pageId", params: { pageId: page.id } })}
    >
      {youtubeId
        ? (
          <>
            <div className="page-card-footer">
              <h2 className="page-title">{page.title}</h2>
            </div>
            <img
              className="page-youtube-thumb"
              src={`https://img.youtube.com/vi/${youtubeId}/mqdefault.jpg`}
              alt={page.title}
            />
          </>
        )
        : (
          <>
            <h2 className="page-title">{page.title}</h2>
            <p className="page-description">{stripHtml(page.description)}</p>
          </>
        )}
    </div>
  );
}

import { useNavigate } from "@tanstack/react-router";
import type { Page } from "../types.ts";

type Props = {
  page: Page;
};

export function PageCard({ page }: Props) {
  const navigate = useNavigate();

  return (
    <div
      className="page-card"
      onClick={() =>
        navigate({ to: "/pages/$pageId", params: { pageId: page.id } })}
    >
      <h2 className="page-title">{page.title}</h2>
      <p className="page-description">{page.description}</p>
    </div>
  );
}

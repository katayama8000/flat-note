import { createFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { PageCard } from "../components/PageCard.tsx";
import type { Page } from "../types.ts";

export const Route = createFileRoute("/")({
  component: Home,
});

function Home() {
  const [pages, setPages] = useState<Page[]>([]);

  useEffect(() => {
    invoke<Page[]>("get_pages").then(setPages);
  }, []);

  return (
    <div className="home">
      <div className="page-grid">
        {pages.map((page) => <PageCard key={page.id} page={page} />)}
      </div>
    </div>
  );
}

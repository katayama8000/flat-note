import { useEffect, useState } from "react";
import "../App.css";
import {
  getPageCount,
  getPages,
  searchPages,
  SortBy,
} from "../features/pages/api/pageApi.ts";
import { PageCard } from "../features/pages/components/PageCard.tsx";
import type { Page } from "../features/pages/types/page.ts";
import { useGlobalSearch } from "../hooks/useGlobalSearch.tsx";

export const HomePage = () => {
  const [pages, setPages] = useState<Page[]>([]);
  const [pageCount, setPageCount] = useState<number>(0);
  const [sortBy, setSortBy] = useState<SortBy>("updatedAt");
  const { query } = useGlobalSearch();

  useEffect(() => {
    getPageCount().then(setPageCount);
  }, []);

  useEffect(() => {
    const keyword = query.trim();
    const timer = globalThis.setTimeout(() => {
      if (keyword) {
        searchPages(keyword, sortBy, 100)
          .then(setPages)
          .catch(() => setPages([]));
        return;
      }

      getPages(sortBy).then(setPages);
    }, 180);

    return () => globalThis.clearTimeout(timer);
  }, [query, sortBy]);

  return (
    <div className="home">
      <div className="toolbar">
        <select
          value={sortBy}
          onChange={(e) => setSortBy(e.target.value as SortBy)}
        >
          <option value="updatedAt">UpdateAt</option>
          <option value="createdAt">CreateAt</option>
        </select>
      </div>
      <div className="page-grid">
        {pages.map((page) => <PageCard key={page.id} page={page} />)}
      </div>
      <div className="page-count">
        {pages.length} / {pageCount} pages
      </div>
    </div>
  );
};

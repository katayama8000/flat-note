import { useEffect, useState } from "react";
import "../App.css";
import {
  getPageCount,
  getPages,
  SortBy,
} from "../features/pages/api/pageApi.ts";
import { PageCard } from "../features/pages/components/PageCard.tsx";
import type { Page } from "../features/pages/types/page.ts";

export const HomePage = () => {
  const [pages, setPages] = useState<Page[]>([]);
  const [pageCount, setPageCount] = useState<number>(0);
  const [sortBy, setSortBy] = useState<SortBy>("updatedAt");

  useEffect(() => {
    getPages(sortBy).then(setPages);
    getPageCount().then(setPageCount);
  }, [sortBy]);

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
      <div className="page-count">{pageCount} pages</div>
    </div>
  );
};

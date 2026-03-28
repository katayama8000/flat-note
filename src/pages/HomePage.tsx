import { useEffect, useState } from "react";
import "../App.css";
import { getPageCount, getPages } from "../features/pages/api/pageApi.ts";
import { PageCard } from "../features/pages/components/PageCard.tsx";
import type { Page } from "../features/pages/types/page.ts";

const HomePage = () => {
  const [pages, setPages] = useState<Page[]>([]);
  const [pageCount, setPageCount] = useState<number>(0);

  useEffect(() => {
    getPages().then(setPages);
    getPageCount().then(setPageCount);
  }, []);

  return (
    <div className="home">
      <div className="page-grid">
        {pages.map((page) => <PageCard key={page.id} page={page} />)}
      </div>
      <div className="page-count">{pageCount} pages</div>
    </div>
  );
};

export default HomePage;

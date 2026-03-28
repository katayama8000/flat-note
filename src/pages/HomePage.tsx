import { useEffect, useState } from "react";
import "../App.css";
import { getPages } from "../features/pages/api/pageApi.ts";
import { PageCard } from "../features/pages/components/PageCard.tsx";
import type { Page } from "../features/pages/types/page.ts";

const HomePage = () => {
  const [pages, setPages] = useState<Page[]>([]);

  useEffect(() => {
    getPages().then(setPages);
  }, []);

  return (
    <div className="home">
      <div className="page-grid">
        {pages.map((page) => <PageCard key={page.id} page={page} />)}
      </div>
    </div>
  );
};

export default HomePage;

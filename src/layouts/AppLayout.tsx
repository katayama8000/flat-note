import { Outlet, useNavigate } from "@tanstack/react-router";
import { useEffect, useMemo, useState } from "react";
import { suggestPageTitles } from "../features/pages/api/pageApi.ts";
import { GlobalSearchProvider } from "../hooks/useGlobalSearch.tsx";

const SEARCH_HISTORY_KEY = "flat-note-search-history";
const SEARCH_HISTORY_LIMIT = 8;

const loadHistory = (): string[] => {
  const raw = globalThis.localStorage.getItem(SEARCH_HISTORY_KEY);
  if (!raw) return [];

  try {
    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];
    return parsed.filter((item): item is string => typeof item === "string");
  } catch {
    return [];
  }
};

const saveHistory = (items: string[]) => {
  globalThis.localStorage.setItem(SEARCH_HISTORY_KEY, JSON.stringify(items));
};

type HeaderProps = {
  query: string;
  setQuery: (value: string) => void;
};

function Header({ query, setQuery }: HeaderProps) {
  const navigate = useNavigate();
  const [history, setHistory] = useState<string[]>([]);
  const [remoteSuggestions, setRemoteSuggestions] = useState<string[]>([]);
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    setHistory(loadHistory());
  }, []);

  useEffect(() => {
    const keyword = query.trim();
    if (!keyword) {
      setRemoteSuggestions([]);
      return;
    }

    const timer = globalThis.setTimeout(() => {
      suggestPageTitles(keyword, 6)
        .then(setRemoteSuggestions)
        .catch(() => setRemoteSuggestions([]));
    }, 160);

    return () => globalThis.clearTimeout(timer);
  }, [query]);

  const addHistory = (term: string) => {
    const normalized = term.trim();
    if (!normalized) return;

    const next = [normalized, ...history.filter((item) => item !== normalized)]
      .slice(0, SEARCH_HISTORY_LIMIT);
    setHistory(next);
    saveHistory(next);
  };

  const matchingHistory = useMemo(() => {
    const keyword = query.trim().toLowerCase();
    if (!keyword) return history;
    return history.filter((item) => item.toLowerCase().includes(keyword));
  }, [history, query]);

  const suggestions = useMemo(() => {
    const combined = [...remoteSuggestions, ...matchingHistory];
    const seen = new Set<string>();
    return combined.filter((item) => {
      const key = item.toLowerCase();
      if (seen.has(key)) return false;
      seen.add(key);
      return true;
    }).slice(0, 8);
  }, [remoteSuggestions, matchingHistory]);

  const onSubmit = () => {
    addHistory(query);
    setIsOpen(true);
  };

  return (
    <header className="app-header">
      <div className="app-header-left" onClick={() => navigate({ to: "/" })}>
        <div className="app-header-icon">f</div>
        <span className="app-header-title">flat note</span>
      </div>
      <div className="app-header-center">
        <button
          className="app-header-create-btn"
          aria-label="New page"
          onClick={() =>
            navigate({ to: "/pages/$pageId", params: { pageId: "new" } })}
          type="button"
        >
          +
        </button>
        <div className="app-header-search-wrap">
          <input
            className="app-header-search"
            type="text"
            placeholder="Search..."
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setIsOpen(true);
            }}
            onFocus={() => setIsOpen(true)}
            onBlur={() => setTimeout(() => setIsOpen(false), 120)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                onSubmit();
              }
            }}
          />
          {isOpen && (
            <div className="search-suggestion-panel">
              {suggestions.length > 0
                ? suggestions.map((item) => (
                  <button
                    key={item}
                    className="search-suggestion-item"
                    type="button"
                    onMouseDown={(e) => e.preventDefault()}
                    onClick={() => {
                      setQuery(item);
                      addHistory(item);
                      setIsOpen(false);
                    }}
                  >
                    {item}
                  </button>
                ))
                : <div className="search-suggestion-empty">No suggestions</div>}
            </div>
          )}
        </div>
      </div>
    </header>
  );
}

export function AppLayout() {
  const [query, setQuery] = useState("");

  return (
    <GlobalSearchProvider value={{ query, setQuery }}>
      <Header query={query} setQuery={setQuery} />
      <Outlet />
    </GlobalSearchProvider>
  );
}

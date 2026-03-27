import { useNavigate } from "@tanstack/react-router";

export function Header() {
  const navigate = useNavigate();

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
        <input
          className="app-header-search"
          type="text"
          placeholder="Search..."
          readOnly
        />
      </div>
    </header>
  );
}

import "./App.css";
import { PageCard } from "./components/PageCard";
import { MOCK_PAGES } from "./mocks";

function App() {
  return (
    <div className="home">
      <div className="page-grid">
        {MOCK_PAGES.map((page) => <PageCard key={page.id} page={page} />)}
      </div>
    </div>
  );
}

export default App;

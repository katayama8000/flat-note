import { useNavigate } from "@tanstack/react-router";
import "../App.css";

export const AboutPage = () => {
  const navigate = useNavigate();

  return (
    <div className="about-page">
      <div className="about-container">
        <button
          className="about-back-btn"
          onClick={() => navigate({ to: "/" })}
          type="button"
        >
          ← Back
        </button>

        <div className="about-header">
          <div className="about-icon">f</div>
          <h1>flat-note</h1>
        </div>

        <div className="about-version">
          <p>Version 0.1.0</p>
        </div>

        <section className="about-section">
          <h2>What is flat-note?</h2>
          <p>
            flat-note is a lightweight, fast, and feature-rich note-taking
            application designed for users who want a simple yet powerful way to
            organize their thoughts and ideas. Built with modern web
            technologies and Tauri, it provides a native desktop experience with
            cloud synchronization capabilities.
          </p>
        </section>

        <section className="about-section">
          <h2>Features</h2>
          <ul className="about-features">
            <li>Rich Text Editing with Markdown support</li>
            <li>Full-Text Search across your entire note library</li>
            <li>Auto-Save functionality (every 1.5 seconds)</li>
            <li>Task Management with nested task lists</li>
            <li>Code Syntax Highlighting for multiple languages</li>
            <li>Table Support with resizable columns</li>
            <li>YouTube Embed support</li>
            <li>Cloud Sync via Turso database</li>
            <li>Keyboard Shortcuts (Cmd/Ctrl+S to save)</li>
          </ul>
        </section>

        <section className="about-section">
          <h2>Technology Stack</h2>
          <div className="about-tech-grid">
            <div className="about-tech-item">
              <h3>Frontend</h3>
              <p>React with TanStack Router</p>
              <p>Tiptap for rich text editing</p>
            </div>
            <div className="about-tech-item">
              <h3>Backend</h3>
              <p>Rust with Tauri</p>
              <p>Domain-Driven Design architecture</p>
            </div>
            <div className="about-tech-item">
              <h3>Database</h3>
              <p>SQLite (local development)</p>
              <p>Turso (cloud storage)</p>
            </div>
          </div>
        </section>

        <section className="about-section">
          <h2>Getting Started</h2>
          <ul>
            <li>
              Press the <strong>+</strong> button to create a new note
            </li>
            <li>Use the search bar to find notes quickly</li>
            <li>
              Type <strong>/table</strong> and press Enter to insert a table
            </li>
            <li>
              Press <strong>Cmd/Ctrl+S</strong> to manually save
            </li>
            <li>Drag and drop content to organize your notes</li>
          </ul>
        </section>

        <section className="about-section">
          <h2>Tips & Tricks</h2>
          <ul>
            <li>Notes auto-save every 1.5 seconds automatically</li>
            <li>Use task lists to create checklists within notes</li>
            <li>
              Code blocks support syntax highlighting for 200+ languages
            </li>
            <li>Search history is saved locally on your computer</li>
            <li>All your data is stored securely</li>
          </ul>
        </section>

        <section className="about-footer">
          <p>
            Built with ❤️ using Tauri, React, and TypeScript
          </p>
        </section>
      </div>
    </div>
  );
};

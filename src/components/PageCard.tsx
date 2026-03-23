import type { Page } from "../types";

type Props = {
    page: Page;
};

export function PageCard({ page }: Props) {
    return (
        <div className="page-card">
            <h2 className="page-title">{page.title}</h2>
            <p className="page-description">{page.description}</p>
        </div>
    );
}

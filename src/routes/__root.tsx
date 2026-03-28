import { createRootRoute } from "@tanstack/react-router";
import { AppLayout } from "../layouts/AppLayout.tsx";

export const Route = createRootRoute({
  component: () => <AppLayout />,
});

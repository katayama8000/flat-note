import { invoke } from "@tauri-apps/api/core";
import type { Page } from "../types/page.ts";

export type SortBy = "createdAt" | "updatedAt";

export const getPages = (sortBy: SortBy) =>
  invoke<Page[]>("get_pages", { sortBy });

export const searchPages = (query: string, sortBy: SortBy, limit = 100) =>
  invoke<Page[]>("search_pages", { query, sortBy, limit });

export const suggestPageTitles = (query: string, limit = 6) =>
  invoke<string[]>("suggest_page_titles", { query, limit });

export const getPageCount = () => invoke<number>("get_page_count");

export const getPage = (id: string) => invoke<Page | null>("get_page", { id });

export const createPage = (title: string) =>
  invoke<Page>("create_page", { title });

export const updateTitle = (id: string, title: string) =>
  invoke("update_title", { id, title });

export const updateTitleDirect = (id: string, title: string) =>
  invoke("update_title_direct", { id, title });

export const updatePage = (id: string, description: string) =>
  invoke("update_page", { id, description });

export const updatePageDirect = (id: string, description: string) =>
  invoke("update_page_direct", { id, description });

import { createContext, useContext } from "react";
import type { ReactNode } from "react";

type GlobalSearchContextValue = {
  query: string;
  setQuery: (value: string) => void;
};

const GlobalSearchContext = createContext<GlobalSearchContextValue | null>(
  null,
);

type ProviderProps = {
  value: GlobalSearchContextValue;
  children: ReactNode;
};

export const GlobalSearchProvider = ({ value, children }: ProviderProps) => {
  return (
    <GlobalSearchContext.Provider value={value}>
      {children}
    </GlobalSearchContext.Provider>
  );
};

export const useGlobalSearch = () => {
  const context = useContext(GlobalSearchContext);
  if (!context) {
    throw new Error("useGlobalSearch must be used within GlobalSearchProvider");
  }
  return context;
};

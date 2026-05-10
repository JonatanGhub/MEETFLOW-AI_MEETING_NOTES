"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { NextIntlClientProvider, type AbstractIntlMessages } from "next-intl";
import { useState, type ReactNode, useEffect } from "react";
import { Toaster } from "sonner";
import { defaultLocale, type Locale } from "@/i18n/config";

// ─── Props ────────────────────────────────────────────────────────────────────

interface ProvidersProps {
  children: ReactNode;
}

// ─── QueryClient factory (one per mount) ──────────────────────────────────────

function makeQueryClient(): QueryClient {
  return new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 30_000,
        retry: 1,
        refetchOnWindowFocus: false,
      },
    },
  });
}

// ─── Locale provider (client-side, stored in localStorage) ───────────────────

function useLocaleMessages(locale: Locale) {
  const [messages, setMessages] = useState<AbstractIntlMessages>({});

  useEffect(() => {
    import(`../messages/${locale}.json`)
      .then((mod: { default: AbstractIntlMessages }) => setMessages(mod.default))
      .catch(() => {
        import("../messages/en.json").then((mod: { default: AbstractIntlMessages }) =>
          setMessages(mod.default)
        );
      });
  }, [locale]);

  return messages;
}

// ─── Root Providers ───────────────────────────────────────────────────────────

export function Providers({ children }: ProvidersProps) {
  const [queryClient] = useState<QueryClient>(makeQueryClient);
  const [locale, setLocale] = useState<Locale>(defaultLocale);
  const messages = useLocaleMessages(locale);

  // Read locale from localStorage on mount
  useEffect(() => {
    const stored = localStorage.getItem("meetflow-locale") as Locale | null;
    if (stored && (stored === "en" || stored === "es")) {
      setLocale(stored);
    }
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <NextIntlClientProvider locale={locale} messages={messages}>
        {children}
        <Toaster
          position="bottom-right"
          theme="dark"
          toastOptions={{
            style: {
              background: "var(--bg-elevated)",
              border: "1px solid var(--border-default)",
              color: "var(--text-primary)",
            },
          }}
        />
      </NextIntlClientProvider>
    </QueryClientProvider>
  );
}

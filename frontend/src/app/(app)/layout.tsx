"use client";

import React from "react";
import { TooltipProvider } from "@/components/ui/tooltip";
import { Sidebar } from "@/components/layout/sidebar";

/**
 * Shell layout for all authenticated/post-onboarding app routes.
 * Renders the sidebar + main content area.
 */
export default function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <TooltipProvider delayDuration={400}>
      <div className="flex h-screen w-screen overflow-hidden bg-[var(--bg-base)]">
        <Sidebar />
        <main className="flex-1 overflow-hidden flex flex-col min-w-0">
          {children}
        </main>
      </div>
    </TooltipProvider>
  );
}

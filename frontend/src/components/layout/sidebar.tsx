"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useTranslations } from "next-intl";
import {
  Mic,
  CalendarDays,
  Settings,
  ChevronLeft,
  ChevronRight,
  Zap,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { useAppStore } from "@/store/app-store";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { Badge } from "@/components/ui/badge";

// ─── Nav items ───────────────────────────────────────────────────────────────

interface NavItem {
  key: string;
  href: string;
  icon: React.ComponentType<{ className?: string }>;
  labelKey: string;
}

const NAV_ITEMS: NavItem[] = [
  { key: "record",   href: "/record",   icon: Mic,          labelKey: "record" },
  { key: "meetings", href: "/meetings", icon: CalendarDays, labelKey: "meetings" },
  { key: "settings", href: "/settings", icon: Settings,     labelKey: "settings" },
];

// ─── Component ───────────────────────────────────────────────────────────────

export function Sidebar() {
  const t = useTranslations("navigation");
  const pathname = usePathname();
  const { sidebarCollapsed, toggleSidebar, recordingState } = useAppStore();

  const isRecording = recordingState === "recording" || recordingState === "paused";

  return (
    <aside
      className={cn(
        "flex flex-col h-full bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] transition-all duration-200",
        sidebarCollapsed ? "w-14" : "w-52"
      )}
    >
      {/* ── Logo / brand ── */}
      <div
        className="flex items-center h-12 px-3 border-b border-[var(--border-subtle)] gap-2 shrink-0 overflow-hidden"
        data-tauri-drag-region
      >
        <div className="flex items-center justify-center w-7 h-7 rounded-lg gradient-brand shrink-0">
          <Zap className="w-3.5 h-3.5 text-white" />
        </div>
        {!sidebarCollapsed && (
          <span className="font-semibold text-sm text-[var(--text-primary)] truncate">
            MeetFlow
          </span>
        )}
        {!sidebarCollapsed && isRecording && (
          <Badge variant="recording" className="ml-auto text-[10px] px-1.5 py-0 h-4">
            REC
          </Badge>
        )}
      </div>

      {/* ── Nav items ── */}
      <nav className="flex flex-col gap-0.5 p-2 flex-1 overflow-hidden">
        {NAV_ITEMS.map((item) => {
          const active = pathname.startsWith(item.href);
          const Icon = item.icon;

          const inner = (
            <Link
              key={item.key}
              href={item.href}
              className={cn(
                "flex items-center gap-2.5 h-9 px-2.5 rounded-lg text-sm font-medium transition-colors w-full",
                active
                  ? "bg-[var(--accent-subtle)] text-[var(--accent)]"
                  : "text-[var(--text-tertiary)] hover:bg-[var(--bg-elevated)] hover:text-[var(--text-primary)]"
              )}
            >
              <Icon className={cn("w-4 h-4 shrink-0", item.key === "record" && isRecording && "text-[var(--recording)]")} />
              {!sidebarCollapsed && (
                <span className="truncate">{t(item.labelKey)}</span>
              )}
              {!sidebarCollapsed && item.key === "record" && isRecording && (
                <span className="ml-auto w-1.5 h-1.5 rounded-full bg-[var(--recording)] animate-pulse" />
              )}
            </Link>
          );

          if (sidebarCollapsed) {
            return (
              <Tooltip key={item.key} delayDuration={0}>
                <TooltipTrigger asChild>{inner}</TooltipTrigger>
                <TooltipContent side="right">{t(item.labelKey)}</TooltipContent>
              </Tooltip>
            );
          }

          return inner;
        })}
      </nav>

      {/* ── Collapse toggle ── */}
      <div className="p-2 border-t border-[var(--border-subtle)] shrink-0">
        <button
          onClick={toggleSidebar}
          className="flex items-center justify-center w-full h-8 rounded-lg text-[var(--text-tertiary)] hover:bg-[var(--bg-elevated)] hover:text-[var(--text-primary)] transition-colors"
          aria-label={sidebarCollapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          {sidebarCollapsed ? (
            <ChevronRight className="w-4 h-4" />
          ) : (
            <ChevronLeft className="w-4 h-4" />
          )}
        </button>
      </div>
    </aside>
  );
}

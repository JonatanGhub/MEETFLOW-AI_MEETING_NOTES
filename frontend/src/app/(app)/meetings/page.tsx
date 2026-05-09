"use client";

import React from "react";
import { useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { Mic, Search, CalendarDays, Clock, CheckSquare, TrendingUp } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { cn, formatDuration, formatRelativeDate } from "@/lib/utils";
import { useMeetingList } from "@/hooks/useMeetings";
import type { MeetingCard } from "@/lib/tauri";

// ─── Meeting card item ────────────────────────────────────────────────────────

function MeetingItem({ card, onClick }: { card: MeetingCard; onClick: () => void }) {
  const t = useTranslations("meetings");
  const dateStr = formatRelativeDate(card.startedAt);
  const durationStr = card.durationSec ? formatDuration(card.durationSec) : null;

  return (
    <button
      onClick={onClick}
      className={cn(
        "w-full text-left px-4 py-3 flex flex-col gap-1 rounded-lg transition-colors",
        "hover:bg-[var(--bg-elevated)] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
      )}
    >
      <div className="flex items-start justify-between gap-2">
        <span className="text-sm font-medium text-[var(--text-primary)] truncate flex-1">
          {card.title}
        </span>
        {card.score !== null && (
          <div className="flex items-center gap-1 shrink-0">
            <TrendingUp className="w-3 h-3 text-[var(--text-tertiary)]" />
            <span className="text-xs text-[var(--text-tertiary)]">{card.score}</span>
          </div>
        )}
      </div>

      <div className="flex items-center gap-3 text-xs text-[var(--text-tertiary)]">
        <span className="flex items-center gap-1">
          <CalendarDays className="w-3 h-3" />
          {dateStr}
        </span>
        {durationStr && (
          <span className="flex items-center gap-1">
            <Clock className="w-3 h-3" />
            {durationStr}
          </span>
        )}
        {card.actionItemCount > 0 && (
          <span className="flex items-center gap-1 text-[var(--accent)]">
            <CheckSquare className="w-3 h-3" />
            {card.actionItemCount}
          </span>
        )}
      </div>

      {card.summarySnippet && (
        <p className="text-xs text-[var(--text-tertiary)] truncate mt-0.5">
          {card.summarySnippet}
        </p>
      )}
    </button>
  );
}

// ─── Page ─────────────────────────────────────────────────────────────────────

export default function MeetingsPage() {
  const t = useTranslations("meetings");
  const router = useRouter();
  const [search, setSearch] = React.useState("");
  const { data: meetings, isLoading } = useMeetingList();

  const filtered = React.useMemo(() => {
    if (!meetings) return [];
    if (!search.trim()) return meetings;
    const q = search.toLowerCase();
    return meetings.filter(
      (m) =>
        m.title.toLowerCase().includes(q) ||
        m.summarySnippet?.toLowerCase().includes(q)
    );
  }, [meetings, search]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-sm text-[var(--text-tertiary)]">Loading…</div>
      </div>
    );
  }

  if (!meetings || meetings.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-4 p-8 text-center">
        <div className="w-16 h-16 rounded-2xl bg-[var(--bg-elevated)] flex items-center justify-center">
          <CalendarDays className="w-8 h-8 text-[var(--text-tertiary)]" />
        </div>
        <div>
          <h2 className="text-base font-semibold text-[var(--text-primary)]">
            {t("empty.title")}
          </h2>
          <p className="text-sm text-[var(--text-tertiary)] mt-1">
            {t("empty.description")}
          </p>
        </div>
        <Button onClick={() => router.push("/record")} className="gap-2">
          <Mic className="w-4 h-4" />
          {t("empty.cta")}
        </Button>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* ── Header ── */}
      <div className="px-4 pt-4 pb-3 border-b border-[var(--border-subtle)] shrink-0">
        <h1 className="text-base font-semibold text-[var(--text-primary)] mb-3">
          {t("title")}
        </h1>
        <div className="relative">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-[var(--text-tertiary)]" />
          <Input
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder={t("search.placeholder")}
            className="pl-8 h-8 text-xs"
          />
        </div>
      </div>

      {/* ── List ── */}
      <ScrollArea className="flex-1">
        <div className="p-2">
          {filtered.length === 0 ? (
            <p className="text-xs text-[var(--text-tertiary)] text-center py-8">
              No meetings match your search.
            </p>
          ) : (
            filtered.map((card) => (
              <MeetingItem
                key={card.id}
                card={card}
                onClick={() => router.push(`/meetings/${card.id}`)}
              />
            ))
          )}
        </div>
      </ScrollArea>
    </div>
  );
}

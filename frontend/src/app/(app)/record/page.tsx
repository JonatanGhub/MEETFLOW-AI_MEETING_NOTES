"use client";

import React, { useState, useRef } from "react";
import { useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { Mic, MicOff, Pause, Play, Square, Clock } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn, formatDuration } from "@/lib/utils";
import { useRecording } from "@/hooks/useRecording";

export default function RecordPage() {
  const t = useTranslations("recording");
  const router = useRouter();
  const { start, stop, pause, resume, recordingState, durationSeconds } = useRecording();
  const [meetingTitle, setMeetingTitle] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const isIdle = recordingState === "idle";
  const isRecording = recordingState === "recording";
  const isPaused = recordingState === "paused";
  const isStopping = recordingState === "stopping";

  const handleStart = async () => {
    const title = meetingTitle.trim() || `Meeting ${new Date().toLocaleString()}`;
    await start(title);
  };

  const handleStop = async () => {
    const meetingId = await stop();
    if (meetingId) {
      router.push(`/meetings/${meetingId}`);
    }
  };

  const formattedTime = formatDuration(durationSeconds);

  return (
    <div className="flex flex-col items-center justify-center h-full gap-8 p-8">
      {/* ── Duration display ── */}
      <div className="text-center">
        <div
          className={cn(
            "text-7xl font-mono font-light tabular-nums tracking-tight transition-colors",
            isRecording && "text-[var(--text-primary)]",
            isPaused && "text-[var(--text-tertiary)]",
            isIdle && "text-[var(--text-tertiary)]"
          )}
        >
          {formattedTime}
        </div>

        {/* Status label */}
        <div
          className={cn(
            "flex items-center justify-center gap-2 mt-3 text-sm font-medium",
            isRecording && "text-[var(--recording)]",
            isPaused && "text-[var(--warning)]",
            isStopping && "text-[var(--text-tertiary)]",
            isIdle && "text-[var(--text-tertiary)]"
          )}
        >
          {isRecording && (
            <span className="w-2 h-2 rounded-full bg-[var(--recording)] animate-pulse" />
          )}
          {isRecording && t("status.recording")}
          {isPaused && t("status.paused")}
          {isStopping && t("status.stopping")}
          {isIdle && t("status.idle")}
        </div>
      </div>

      {/* ── Meeting name input (only when idle) ── */}
      {isIdle && (
        <div className="w-full max-w-sm">
          <Input
            ref={inputRef}
            value={meetingTitle}
            onChange={(e) => setMeetingTitle(e.target.value)}
            placeholder={t("meeting_name.placeholder")}
            className="text-center text-base h-11 bg-[var(--bg-elevated)]"
            onKeyDown={(e) => {
              if (e.key === "Enter") handleStart();
            }}
            maxLength={120}
          />
        </div>
      )}

      {/* ── Primary record button ── */}
      <div className="relative">
        {(isRecording || isPaused) && (
          <span className="absolute inset-0 rounded-full bg-[var(--recording)]/20 animate-ping" />
        )}
        <button
          onClick={isIdle ? handleStart : isPaused ? resume : isRecording ? pause : undefined}
          disabled={isStopping}
          className={cn(
            "relative w-24 h-24 rounded-full flex items-center justify-center transition-all duration-200 focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-[var(--ring)]",
            isIdle && "bg-[var(--accent)] hover:bg-[var(--accent-hover)] active:scale-95 recording-glow",
            isRecording && "bg-[var(--recording)] hover:bg-[var(--recording)]/80 active:scale-95 recording-glow",
            isPaused && "bg-[var(--bg-elevated)] border-2 border-[var(--border-strong)] hover:bg-[var(--bg-overlay)] active:scale-95",
            isStopping && "bg-[var(--bg-elevated)] cursor-not-allowed opacity-50"
          )}
          aria-label={isIdle ? t("button.start") : isRecording ? t("button.pause") : t("button.resume")}
        >
          {isIdle && <Mic className="w-9 h-9 text-white" />}
          {isRecording && <Pause className="w-9 h-9 text-white" />}
          {isPaused && <Play className="w-9 h-9 text-[var(--text-primary)]" />}
          {isStopping && <Clock className="w-9 h-9 text-[var(--text-tertiary)] animate-spin" />}
        </button>
      </div>

      {/* ── Stop button (when active) ── */}
      {(isRecording || isPaused) && !isStopping && (
        <Button
          variant="outline"
          size="lg"
          onClick={handleStop}
          className="gap-2"
        >
          <Square className="w-4 h-4" />
          {t("button.stop")}
        </Button>
      )}

      {/* ── Hint text ── */}
      {isIdle && (
        <p className="text-xs text-[var(--text-tertiary)] text-center max-w-xs">
          Audio stays on your device. Transcription runs locally via Whisper.
        </p>
      )}
    </div>
  );
}

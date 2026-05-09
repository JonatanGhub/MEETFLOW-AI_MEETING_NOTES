"use client";

import { useCallback, useEffect, useRef } from "react";
import { toast } from "sonner";
import { useAppStore } from "@/store/app-store";
import {
  startRecording,
  stopRecording,
  pauseRecording,
  resumeRecording,
  getRecordingStatus,
} from "@/lib/tauri";

/**
 * Hook that owns all recording lifecycle logic.
 * Polls Rust backend every second for duration updates.
 * Listens to Tauri events for real-time transcript updates.
 */
export function useRecording() {
  const {
    recordingState,
    setRecordingState,
    currentMeetingId,
    setCurrentMeetingId,
    recordingDurationSeconds,
    setRecordingDuration,
  } = useAppStore();

  const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // ── Sync duration from Rust every second while recording ──
  useEffect(() => {
    if (recordingState === "recording" || recordingState === "paused") {
      pollRef.current = setInterval(async () => {
        try {
          const status = await getRecordingStatus();
          setRecordingDuration(status.durationSeconds);
          // Sync paused state
          if (status.isPaused && recordingState === "recording") {
            setRecordingState("paused");
          } else if (!status.isPaused && recordingState === "paused") {
            setRecordingState("recording");
          }
        } catch {
          // ignore poll errors
        }
      }, 1000);
    } else {
      if (pollRef.current) {
        clearInterval(pollRef.current);
        pollRef.current = null;
      }
    }
    return () => {
      if (pollRef.current) {
        clearInterval(pollRef.current);
        pollRef.current = null;
      }
    };
  }, [recordingState, setRecordingDuration, setRecordingState]);

  // ── Start ──────────────────────────────────────────────────────────────────
  const start = useCallback(async (title: string) => {
    if (recordingState !== "idle") return;
    try {
      setRecordingState("recording");
      const meetingId = await startRecording(title);
      setCurrentMeetingId(meetingId);
      setRecordingDuration(0);
    } catch (err) {
      setRecordingState("idle");
      toast.error(`Failed to start recording: ${err}`);
    }
  }, [recordingState, setRecordingState, setCurrentMeetingId, setRecordingDuration]);

  // ── Stop ───────────────────────────────────────────────────────────────────
  const stop = useCallback(async (): Promise<string | null> => {
    if (recordingState === "idle") return null;
    try {
      setRecordingState("stopping");
      const meetingId = await stopRecording();
      setRecordingState("idle");
      setCurrentMeetingId(null);
      setRecordingDuration(0);
      return meetingId;
    } catch (err) {
      setRecordingState("idle");
      toast.error(`Failed to stop recording: ${err}`);
      return null;
    }
  }, [recordingState, setRecordingState, setCurrentMeetingId, setRecordingDuration]);

  // ── Pause ──────────────────────────────────────────────────────────────────
  const pause = useCallback(async () => {
    if (recordingState !== "recording") return;
    try {
      await pauseRecording();
      setRecordingState("paused");
    } catch (err) {
      toast.error(`Failed to pause: ${err}`);
    }
  }, [recordingState, setRecordingState]);

  // ── Resume ─────────────────────────────────────────────────────────────────
  const resume = useCallback(async () => {
    if (recordingState !== "paused") return;
    try {
      await resumeRecording();
      setRecordingState("recording");
    } catch (err) {
      toast.error(`Failed to resume: ${err}`);
    }
  }, [recordingState, setRecordingState]);

  return {
    recordingState,
    currentMeetingId,
    durationSeconds: recordingDurationSeconds,
    start,
    stop,
    pause,
    resume,
    isRecording: recordingState === "recording",
    isPaused: recordingState === "paused",
    isIdle: recordingState === "idle",
    isStopping: recordingState === "stopping",
  };
}

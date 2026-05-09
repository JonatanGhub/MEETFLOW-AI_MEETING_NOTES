import { create } from "zustand";
import { persist } from "zustand/middleware";

// ─── Types ────────────────────────────────────────────────────────────────────

export type RecordingState = "idle" | "recording" | "paused" | "stopping";

export interface AppState {
  // Onboarding
  onboardingComplete: boolean;
  setOnboardingComplete: (done: boolean) => void;

  // Recording
  recordingState: RecordingState;
  setRecordingState: (state: RecordingState) => void;
  currentMeetingId: string | null;
  setCurrentMeetingId: (id: string | null) => void;
  recordingDurationSeconds: number;
  setRecordingDuration: (seconds: number) => void;

  // UI
  sidebarCollapsed: boolean;
  toggleSidebar: () => void;
  activeMeetingId: string | null;
  setActiveMeetingId: (id: string | null) => void;
}

// ─── Store ────────────────────────────────────────────────────────────────────

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      // Onboarding
      onboardingComplete: false,
      setOnboardingComplete: (done) => set({ onboardingComplete: done }),

      // Recording
      recordingState: "idle",
      setRecordingState: (state) => set({ recordingState: state }),
      currentMeetingId: null,
      setCurrentMeetingId: (id) => set({ currentMeetingId: id }),
      recordingDurationSeconds: 0,
      setRecordingDuration: (seconds) => set({ recordingDurationSeconds: seconds }),

      // UI
      sidebarCollapsed: false,
      toggleSidebar: () => set((s) => ({ sidebarCollapsed: !s.sidebarCollapsed })),
      activeMeetingId: null,
      setActiveMeetingId: (id) => set({ activeMeetingId: id }),
    }),
    {
      name: "meetflow-app-store",
      // Only persist these keys
      partialize: (state) => ({
        onboardingComplete: state.onboardingComplete,
        sidebarCollapsed: state.sidebarCollapsed,
      }),
    }
  )
);

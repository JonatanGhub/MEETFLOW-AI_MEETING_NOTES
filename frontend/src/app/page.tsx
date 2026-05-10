"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";

/**
 * Root page — redirects to /record or /onboarding based on whether
 * the user has completed onboarding (read from Zustand persisted store).
 */
export default function RootPage() {
  const router = useRouter();

  useEffect(() => {
    try {
      const raw = localStorage.getItem("meetflow-app-store");
      const state = raw ? (JSON.parse(raw) as { state?: { onboardingComplete?: boolean } }) : null;
      const onboardingDone = state?.state?.onboardingComplete === true;
      router.replace(onboardingDone ? "/record" : "/onboarding");
    } catch {
      router.replace("/onboarding");
    }
  }, [router]);

  return (
    <div className="flex h-screen items-center justify-center bg-[var(--bg-base)]">
      <div className="h-5 w-5 animate-spin rounded-full border-2 border-[var(--border-default)] border-t-[var(--accent)]" />
    </div>
  );
}

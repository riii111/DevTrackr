"use client";

import { useState, useEffect, useCallback } from "react";
import { WorkLog } from "@/types/worklog";

export const useAutoSave = (state: Partial<WorkLog>) => {
  const [lastAutoSave, setLastAutoSave] = useState<Date | null>(null);

  const save = useCallback(() => {
    localStorage.setItem("workLog_autosave", JSON.stringify(state));
    setLastAutoSave(new Date());
  }, [state]);

  useEffect(() => {
    if (state.start_time && !state.end_time) {
      const timer = setInterval(save, 5 * 60 * 1000);
      return () => clearInterval(timer);
    }
  }, [state, save]);

  return { save, lastAutoSave };
};

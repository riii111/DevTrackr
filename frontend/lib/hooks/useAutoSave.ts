"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import { WorkLog } from "@/types/worklog";
import { updateWorkLogAction } from "@/lib/actions/worklog";
import { useToast } from "@/lib/hooks/use-toast";

interface AutoSaveState extends Partial<WorkLog> {
  break_time?: number;
  end_time?: string;
  workLogId?: string | null;
  project_id?: string;
  memo?: string;
}

export const useAutoSave = (state: AutoSaveState) => {
  const { toast } = useToast();
  const [lastAutoSave, setLastAutoSave] = useState<Date | null>(null);
  const [isDirty, setIsDirty] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const saveTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const lastSavedStateRef = useRef<string>("");

  // デバウンス用のタイマー
  const debounceTimerRef = useRef<NodeJS.Timeout | null>(null);

  // ローカルストレージへの保存
  // ブラウザ更新・クラッシュや不意の操作に対するリカバリーとしてローカルストレージを採用
  const saveToLocalStorage = useCallback((data: AutoSaveState) => {
    if (!data.project_id) return;

    try {
      localStorage.setItem(
        `workLog_autosave_${data.project_id}`,
        JSON.stringify({
          ...data,
          lastModified: new Date().toISOString(),
        })
      );
    } catch (error) {
      console.error("ローカルストレージへの保存に失敗しました:", error);
    }
  }, []);

  // APIへの保存
  const saveToAPI = useCallback(
    async (data: AutoSaveState) => {
      if (!data.project_id || !data.workLogId) return;

      try {
        setIsSaving(true);
        const result = await updateWorkLogAction(data.workLogId, {
          project_id: data.project_id,
          start_time: data.start_time || "",
          end_time: data.end_time || undefined,
          memo: data.memo || undefined,
          break_time: data.break_time,
        });

        if (!result.success) {
          throw new Error(result.error);
        }

        // API保存成功時にローカルストレージをクリア
        localStorage.removeItem(`workLog_autosave_${data.project_id}`);
        setLastAutoSave(new Date());
        setIsDirty(false);

        toast({
          description: "保存しました",
          duration: 2000,
        });
      } catch (error) {
        console.error("保存に失敗しました:", error);
        toast({
          variant: "destructive",
          description: "保存に失敗しました",
        });
      } finally {
        setIsSaving(false);
      }
    },
    [toast]
  );

  // 変更検知とデバウンス処理
  useEffect(() => {
    if (!state.project_id || !state.workLogId) return;

    const currentState = JSON.stringify({
      memo: state.memo,
      break_time: state.break_time,
      end_time: state.end_time,
      start_time: state.start_time,
    });

    if (currentState !== lastSavedStateRef.current) {
      setIsDirty(true);
      // 即時にローカルストレージへ保存
      saveToLocalStorage(state);

      // 既存のタイマーをクリア
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }

      // start_timeの変更は即時保存
      // end_timeの設定も即時保存
      if (
        (state.start_time &&
          !lastSavedStateRef.current.includes("start_time")) ||
        state.end_time
      ) {
        saveToAPI(state);
        lastSavedStateRef.current = currentState;
      } else {
        // その他の変更は1000msでデバウンス
        debounceTimerRef.current = setTimeout(() => {
          saveToAPI(state);
          lastSavedStateRef.current = currentState;
        }, 1000); // デバウンス時間を1000msに短縮
      }
    }

    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, [state, saveToLocalStorage, saveToAPI]);

  return {
    lastAutoSave,
    isDirty,
    isSaving,
  };
};

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
        console.log("success!! memo : ", data.memo);

        // 成功時はDialog内に出力されるのでtoast不要
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
    });

    if (currentState !== lastSavedStateRef.current) {
      setIsDirty(true);
      // 即時にローカルストレージへ保存
      saveToLocalStorage(state);

      // 既存のタイマーをクリア
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }

      // デバウンス処理
      debounceTimerRef.current = setTimeout(() => {
        saveToAPI(state);
        lastSavedStateRef.current = currentState;
      }, 1500);
    }

    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, [state, saveToLocalStorage, saveToAPI]);

  // 復元用の関数
  const restoreState = useCallback((lastModified: Date) => {
    setLastAutoSave(lastModified);
    setIsDirty(true);
  }, []);

  return {
    lastAutoSave,
    isDirty,
    isSaving,
    restoreState, // 復元用の関数を公開
  };
};

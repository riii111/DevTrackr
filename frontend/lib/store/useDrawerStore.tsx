"use client";
import { createContext, useCallback, useState, useEffect } from "react";
import { useRouter, usePathname, useSearchParams } from "next/navigation";
import { createExternalPromise } from "@/lib/utils/promiseUtils";

type DrawerType = "main" | "sub";
type EventDataVariant = "event" | "task" | "todo";

interface DrawerState {
  isOpen: boolean;
  id?: string;
  type?: EventDataVariant;
  drawerClosePromise?: Promise<void>;
  resolve?: (value: void | PromiseLike<void>) => void;
}

interface DrawerContextType {
  drawerState: Record<DrawerType, DrawerState>;
  handleOpen: (
    drawer: DrawerType,
    { id, type }: { id: string; type: EventDataVariant }
  ) => Promise<void>;
  handleClose: (drawerType: DrawerType) => Promise<void>;
  onClosed: (drawer: DrawerType) => void;
}

const DrawerContext = createContext<DrawerContextType | undefined>(undefined);

export const DrawerProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [mounted, setMounted] = useState(false);
  const router = useRouter();
  useEffect(() => {
    setMounted(true);
  }, []);
  const searchParams = useSearchParams();
  const pathname = usePathname();

  const [drawerState, setDrawerState] = useState<
    Record<DrawerType, DrawerState>
  >({
    main: {
      isOpen: false,
      id: undefined,
      type: undefined,
      drawerClosePromise: undefined,
      resolve: undefined,
    },
    sub: {
      isOpen: false,
      id: undefined,
      type: undefined,
      drawerClosePromise: undefined,
      resolve: undefined,
    },
  });

  /**
   * ドロワーを開く関数
   * ドロワーが完全に閉じるまでにタイムラグがあるため、
   * 閉じたことを外部に通知するためのPromiseを、ドロワーが開いた段階で定義しておく
   */
  const handleOpen = useCallback(
    async (
      drawer: DrawerType,
      { id, type }: { id: string; type: EventDataVariant }
    ) => {
      if (drawer === "sub" && !drawerState.main.isOpen) {
        throw new Error("mainドロワーが開いていません");
      }

      const { promisify, resolve } = createExternalPromise();

      setDrawerState((prev) => ({
        ...prev,
        [drawer]: {
          ...prev[drawer],
          isOpen: true,
          id,
          type,
          drawerClosePromise: promisify,
          resolve,
        },
      }));

      if (drawer === "main" && router) {
        const params = new URLSearchParams(searchParams);
        params.set(type + "Id", id);
        router.push(`${pathname}?${params.toString()}`);
      }
    },
    [drawerState, router, searchParams, pathname]
  );

  const handleClose = useCallback(
    async (drawerType: DrawerType) => {
      setDrawerState((prev) => ({
        ...prev,
        [drawerType]: { ...prev[drawerType], isOpen: false },
      }));

      if (drawerType === "main") {
        const params = new URLSearchParams(searchParams);
        params.delete("eventId");
        params.delete("taskId");
        params.delete("todoId");
        router.push(`${pathname}?${params.toString()}`);
      }
    },
    [router, searchParams, pathname]
  );

  const onClosed = useCallback(
    (drawer: DrawerType) => {
      setDrawerState((prev) => ({
        ...prev,
        [drawer]: {
          ...prev[drawer],
          id: undefined,
          type: undefined,
        },
      }));
      drawerState[drawer].resolve?.();
    },
    [drawerState]
  );

  if (!mounted) {
    return null;
  }

  return (
    <DrawerContext.Provider
      value={{ drawerState, handleOpen, handleClose, onClosed }}
    >
      {children}
    </DrawerContext.Provider>
  );
};


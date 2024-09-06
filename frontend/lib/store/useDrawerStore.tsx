"use client";
import { createContext, useCallback, useState, useEffect, useContext } from "react";
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
  isFullScreen: boolean;
  setIsFullScreen: (value: boolean) => void;
}

const DrawerContext = createContext<DrawerContextType | undefined>(undefined);

export const useDrawerStore = () => {
  const context = useContext(DrawerContext);
  if (context === undefined) {
    throw new Error('useDrawerStoreはDrawerProvider内で使用する必要があります');
  }
  return context;
};

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

  const [isFullScreen, setIsFullScreen] = useState<boolean>(false)

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
        await router.push(`${pathname}?${params.toString()}`);
        // TODO:何かスナックバーを表示させる？しないなら非同期は不要.
      }
    },
    [drawerState, router, searchParams, pathname]
  );

  const handleClose = useCallback(
    // TODO: useCallbackの依存配列にdrawerState全体を含めているが、必要な部分だけを指定し不要な再レンダリングを抑える
    async (drawerType: DrawerType) => {
      setDrawerState((prev) => ({
        ...prev,
        [drawerType]: { ...prev[drawerType], isOpen: false },
      }));

      setIsFullScreen(false);

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
      value={{ drawerState, handleOpen, handleClose, onClosed, isFullScreen, setIsFullScreen }}
    >
      {children}
    </DrawerContext.Provider>
  );
};


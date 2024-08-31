import { createContext, useContext, useCallback, useState } from "react";
import { useRouter } from "next/router";
import { createExternalPromise } from "@/lib/utils/promiseUtils";

type DrawerType = "main" | "sub";
type EventDataVariant = "event" | "task" | "todo";

interface DrawerState {
  isOpen: boolean;
  id?: string;
  type?: Event;
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
  const router = useRouter();
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

      if (drawer === "main") {
        await router.push(
          {
            query: {
              ...router.query,
              [type + "Id"]: id,
            },
          },
          undefined,
          { shallow: true }
        );
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
    },
    [drawerState, router]
  );


  const handleClose = useCallback(
    async (drawerType: DrawerType) => {
      setDrawerState((prev) => ({
        ...prev,
        [drawerType]: { ...prev[drawerType], isOpen: false },
      }));

      if (drawerType === "main") {
        await router.push(
          {
            query: {
              ...router.query,
              eventId: undefined,
              taskId: undefined,
              todoId: undefined,
            },
          },
          undefined,
          { shallow: true }
        );
      }
    },
    [router]
  );

  const onClosed = useCallback(
    (drawer: DrawerType) => {
      setDrawerState((prev) => ({
        ...prev,
        [drawer]: {
          ...prev,
          [drawer]: {
            ...prev[drawer],
            id: undefined,
            type: undefined,
          },
        },
      }));
      drawerState[drawer].resolve?.();
    },
    [drawerState]
  );

  return (
    <DrawerContext.Provider
      value={{ drawerState, handleOpen, handleClose, onClosed }}
    >
      {children}
    </DrawerContext.Provider>
  );
};


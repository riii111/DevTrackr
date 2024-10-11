"use client";
import { createContext, useCallback, useReducer, useContext, useMemo, useEffect } from "react";
import { useRouter, usePathname, useSearchParams } from "next/navigation";
import { createExternalPromise } from "@/lib/utils/promiseUtils";

type DrawerType = "main" | "sub";
type DataVariant = "company" | "project" | "workLog";

type DrawerAction =
  | { type: "OPEN_DRAWER"; drawer: DrawerType; id: string; dataType: DataVariant }
  | { type: "CLOSE_DRAWER"; drawer: DrawerType }
  | { type: "ON_CLOSED"; drawer: DrawerType }
  | { type: "SET_FULL_SCREEN"; value: boolean }


const drawerReducer = (state: DrawerContextType, action: DrawerAction): DrawerContextType => {
  switch (action.type) {
    case "OPEN_DRAWER": {
      const { promisify, resolve } = createExternalPromise();
      return {
        ...state,
        drawerState: {
          ...state.drawerState,
          [action.drawer]: {
            isOpen: true,
            id: action.id,
            dataType: action.dataType,
            drawerClosePromise: promisify,
            resolve,
          }
        }
      }
    }
    case "CLOSE_DRAWER":
      return {
        ...state,
        drawerState: {
          ...state.drawerState,
          [action.drawer]: { ...state.drawerState[action.drawer], isOpen: false },
        },
        isFullScreen: false,
      };
    case "ON_CLOSED":
      return {
        ...state,
        drawerState: {
          ...state.drawerState,
          [action.drawer]: {
            ...state.drawerState[action.drawer],
            id: undefined,
            type: undefined,
          },
        },
      };
    case 'SET_FULL_SCREEN':
      return { ...state, isFullScreen: action.value };
    default:
      return state;
  }
}

interface DrawerState {
  isOpen: boolean;
  id?: string;
  dataType?: DataVariant;
  drawerClosePromise?: Promise<void>;
  resolve?: (value: void | PromiseLike<void>) => void;
}

interface DrawerContextType {
  drawerState: Record<DrawerType, DrawerState>;
  handleOpen: (
    drawer: DrawerType,
    { id, dataType }: { id: string; dataType: DataVariant }
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
  const [state, dispatch] = useReducer(drawerReducer, {
    drawerState: {
      main: { isOpen: false },
      sub: { isOpen: false },
    },
    isFullScreen: false,
    handleOpen: () => Promise.resolve(),
    handleClose: () => Promise.resolve(),
    onClosed: () => { },
    setIsFullScreen: () => { },
  });

  const router = useRouter();
  const searchParams = useSearchParams();
  const pathname = usePathname();

  // URLからドロワーの状態を同期する
  useEffect(() => {
    // TODO: 種別あとで増えます
    const projectId = searchParams.get('projectId');

    if (projectId) {
      handleOpen('main', { id: projectId, dataType: 'project' });
    } else {
      handleClose('main');
    }
  }, [searchParams]);

  /**
   * ドロワーを開く関数
   * ドロワーが完全に閉じるまでにタイムラグがあるため、
   * 閉じたことを外部に通知するためのPromiseを、ドロワーが開いた段階で定義しておく
   */
  const handleOpen = useCallback(
    async (
      drawer: DrawerType,
      { id, dataType }: { id: string; dataType: DataVariant }
    ) => {
      if (drawer === "sub" && !state.drawerState.main.isOpen) {
        throw new Error("mainドロワーが開いていません");
      }

      const { promisify, resolve } = createExternalPromise();

      dispatch({ type: "OPEN_DRAWER", drawer, id, dataType });

      if (drawer === "main" && router) {
        const params = new URLSearchParams(searchParams);
        params.set(dataType + "Id", id);
        await router.push(`${pathname}?${params.toString()}`);
      }
    },
    [state.drawerState.main.isOpen, router, searchParams, pathname]
  );


  const handleClose = useCallback(
    async (drawerType: DrawerType) => {
      dispatch({ type: 'CLOSE_DRAWER', drawer: drawerType });

      if (drawerType === "main") {
        const params = new URLSearchParams(searchParams);
        params.delete("projectId");
        router.push(`${pathname}?${params.toString()}`);
      }
    },
    [router, searchParams, pathname]
  );

  const onClosed = useCallback(
    (drawer: DrawerType) => {
      dispatch({ type: "ON_CLOSED", drawer });
      state.drawerState[drawer].resolve?.();
    },
    [state.drawerState]
  );

  const setIsFullScreen = useCallback((value: boolean) => {
    dispatch({ type: 'SET_FULL_SCREEN', value });
  }, []);



  const contextValue = useMemo(() => ({
    drawerState: state.drawerState,
    handleOpen,
    handleClose,
    onClosed,
    isFullScreen: state.isFullScreen,
    setIsFullScreen,
  }), [state, handleOpen, handleClose, onClosed, setIsFullScreen])

  return (
    <DrawerContext.Provider
      value={contextValue}
    >
      {children}
    </DrawerContext.Provider>
  );
};


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
    const syncDrawerWithUrl = () => {
      const projectId = searchParams.get('projectId');
      if (projectId) {
        dispatch({ type: "OPEN_DRAWER", drawer: 'main', id: projectId, dataType: 'project' });
      } else {
        dispatch({ type: "CLOSE_DRAWER", drawer: 'main' });
      }
    };

    syncDrawerWithUrl();
  }, [searchParams]);

  const openDrawerWithUrl = useCallback(
    async (drawer: DrawerType, { id, dataType }: { id: string; dataType: DataVariant }) => {
      if (drawer === "main") {
        const params = new URLSearchParams(searchParams);
        params.set(dataType + "Id", id);
        await router.push(`${pathname}?${params.toString()}`);
      }
    },
    [router, searchParams, pathname]
  );

  const closeDrawerWithUrl = useCallback(
    async (drawerType: DrawerType) => {
      if (drawerType === "main") {
        const params = new URLSearchParams(searchParams);
        params.delete("projectId");
        await router.push(`${pathname}?${params.toString()}`);
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
    openDrawerWithUrl,
    closeDrawerWithUrl,
    onClosed,
    isFullScreen: state.isFullScreen,
    setIsFullScreen,
  }), [state, openDrawerWithUrl, closeDrawerWithUrl, onClosed, setIsFullScreen])

  return (
    <DrawerContext.Provider
      value={{
        ...contextValue,
        handleOpen: openDrawerWithUrl,
        handleClose: closeDrawerWithUrl
      }}
    >
      {children}
    </DrawerContext.Provider>
  );
};


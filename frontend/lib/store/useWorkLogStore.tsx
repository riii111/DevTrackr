"use client";
import React, { createContext, useContext, useReducer, ReactNode } from 'react';

export interface WorkLogState {
    // ダイアログの表示制御用の状態
    isOpen: boolean;
    projectId: string | null;
    clickPosition: { x: number; y: number } | null;

    // 作業記録用の状態
    startTime: Date | null;
    endTime: Date | null;
    breakTime: number;
    memo: string;
    isPaused: boolean;
    pauseStartTime: Date | null;
    workLogId: string | null;
}

// アクションの型定義を拡張
export type WorkLogAction =
    | { type: 'OPEN_WORK_LOG'; projectId: string; position: { x: number; y: number } }
    | { type: 'CLOSE_WORK_LOG' }
    | { type: 'SET_START_TIME'; time: Date | null }
    | { type: 'SET_END_TIME'; time: Date | null }
    | { type: 'SET_BREAK_TIME'; time: number }
    | { type: 'SET_MEMO'; memo: string }
    | { type: 'SET_PAUSE_STATUS'; isPaused: boolean; pauseStartTime: Date | null }
    | { type: 'SET_WORK_LOG_ID'; id: string }
    | { type: 'RESET_WORK_LOG' };

const initialState: WorkLogState = {
    isOpen: false,
    projectId: null,
    clickPosition: null,
    startTime: null,
    endTime: null,
    breakTime: 0,
    memo: "",
    isPaused: false,
    pauseStartTime: null,
    workLogId: null,
};

const workLogReducer = (state: WorkLogState, action: WorkLogAction): WorkLogState => {
    switch (action.type) {
        case 'OPEN_WORK_LOG':
            return {
                ...state,
                isOpen: true,
                projectId: action.projectId,
                clickPosition: action.position,
            };
        case 'CLOSE_WORK_LOG':
            return initialState;
        case 'SET_START_TIME':
            return {
                ...state,
                startTime: action.time,
            };
        case 'SET_END_TIME':
            return {
                ...state,
                endTime: action.time,
            };
        case 'SET_BREAK_TIME':
            return {
                ...state,
                breakTime: action.time,
            };
        case 'SET_MEMO':
            return {
                ...state,
                memo: action.memo,
            };
        case 'SET_PAUSE_STATUS':
            return {
                ...state,
                isPaused: action.isPaused,
                pauseStartTime: action.pauseStartTime,
            };
        case 'SET_WORK_LOG_ID':
            return {
                ...state,
                workLogId: action.id,
            };
        case 'RESET_WORK_LOG':
            return {
                ...state,
                startTime: null,
                endTime: null,
                breakTime: 0,
                memo: "",
                isPaused: false,
                pauseStartTime: null,
                workLogId: null,
            };
        default:
            return state;
    }
};

interface WorkLogContextType {
    state: WorkLogState;
    dispatch: React.Dispatch<WorkLogAction>;
}

const WorkLogContext = createContext<WorkLogContextType | undefined>(undefined);

export const WorkLogProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [state, dispatch] = useReducer(workLogReducer, initialState);

    return (
        <WorkLogContext.Provider value={{ state, dispatch }}>
            {children}
        </WorkLogContext.Provider>
    );
};

export const useWorkLog = () => {
    const context = useContext(WorkLogContext);
    if (context === undefined) {
        throw new Error('useWorkLog must be used within a WorkLogProvider');
    }

    const { state, dispatch } = context;

    const actions = {
        openWorkLog: (projectId: string, position: { x: number; y: number }) => {
            dispatch({ type: 'OPEN_WORK_LOG', projectId, position });
        },
        closeWorkLog: () => {
            dispatch({ type: 'CLOSE_WORK_LOG' });
        },
        setStartTime: (time: Date | null) => {
            dispatch({ type: 'SET_START_TIME', time });
        },
        setEndTime: (time: Date | null) => {
            dispatch({ type: 'SET_END_TIME', time });
        },
        setBreakTime: (time: number) => {
            dispatch({ type: 'SET_BREAK_TIME', time });
        },
        setMemo: (memo: string) => {
            dispatch({ type: 'SET_MEMO', memo });
        },
        setPauseStatus: (isPaused: boolean, pauseStartTime: Date | null) => {
            dispatch({ type: 'SET_PAUSE_STATUS', isPaused, pauseStartTime });
        },
        setWorkLogId: (id: string) => {
            dispatch({ type: 'SET_WORK_LOG_ID', id });
        },
        resetWorkLog: () => {
            dispatch({ type: 'RESET_WORK_LOG' });
        },
    };

    return { state, dispatch, ...actions };
};

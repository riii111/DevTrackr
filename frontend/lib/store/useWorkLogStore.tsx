"use client";
import React, { createContext, useContext, useReducer, ReactNode } from 'react';

export type WorkLogState = {
    isOpen: boolean;
    projectId: string | null;
};

export type WorkLogAction =
    | { type: 'OPEN_WORK_LOG'; projectId: string }
    | { type: 'CLOSE_WORK_LOG' };

const initialState: WorkLogState = {
    isOpen: false,
    projectId: null,
};

const workLogReducer = (state: WorkLogState, action: WorkLogAction): WorkLogState => {
    switch (action.type) {
        case 'OPEN_WORK_LOG':
            return { ...state, isOpen: true, projectId: action.projectId };
        case 'CLOSE_WORK_LOG':
            return { ...state, isOpen: false, projectId: null };
        default:
            return state;
    }
};

const WorkLogContext = createContext<{
    state: WorkLogState;
    dispatch: React.Dispatch<WorkLogAction>;
} | undefined>(undefined);

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
    return context;
};
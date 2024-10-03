'use client';

import React, { useState, useEffect } from 'react';
import { Alert, AlertDescription } from "@/components/ui/alert";

const ClientAlert = () => {
    const [generalError, setGeneralError] = useState<string | null>(null);

    useEffect(() => {
        const handleError = (event: Event) => {
            const customEvent = event as CustomEvent<Error>;
            setGeneralError(customEvent.detail.message);
        };

        // エラーメッセージをクリアする関数
        const handleClearError = () => {
            setGeneralError(null);
        };

        window.addEventListener('authError', handleError);
        window.addEventListener('clearAuthError', handleClearError);

        return () => {
            window.removeEventListener('authError', handleError);
            window.removeEventListener('clearAuthError', handleClearError);
        };
    }, []);

    if (!generalError) return null;

    return (
        <Alert variant="destructive" className="mt-4">
            <AlertDescription>{generalError}</AlertDescription>
        </Alert>
    );
};

export default ClientAlert;
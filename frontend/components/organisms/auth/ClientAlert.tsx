'use client';

import React, { useState, useEffect } from 'react';
import { Alert, AlertDescription } from "@/components/ui/alert";

const ClientAlert = () => {
    const [generalError, setGeneralError] = useState<string | null>(null);

    useEffect(() => {
        const handleError = (event: CustomEvent<Error>) => {
            setGeneralError(event.detail.message);
        };

        window.addEventListener('authError' as any, handleError as EventListener);

        return () => {
            window.removeEventListener('authError' as any, handleError as EventListener);
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
"use client"

import React from "react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"

export const ErrorAlert: React.FC<{ error: Error }> = ({ error }) => (
    <Alert variant="destructive">
        <AlertTitle>エラーが発生しました</AlertTitle>
        <AlertDescription>{error.message}</AlertDescription>
    </Alert>
);
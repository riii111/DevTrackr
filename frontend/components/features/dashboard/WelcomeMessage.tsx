"use client";

import { useEffect } from 'react';
import { useToast } from "@/lib/hooks/use-toast";

export const WelcomeMessage: React.FC = () => {
    const { toast } = useToast();

    useEffect(() => {
        const getCookie = (name: string) => {
            const value = `; ${document.cookie}`;
            const parts = value.split(`; ${name}=`);
            if (parts.length === 2) {
                const cookieValue = parts.pop()?.split(';').shift();
                return cookieValue ? decodeURIComponent(cookieValue) : null;
            }
            return null;
        };

        const firstLoginCookie = getCookie('firstLogin');
        if (firstLoginCookie) {
            try {
                const { username, show } = JSON.parse(firstLoginCookie);
                if (show) {
                    toast({
                        variant: "success",
                        title: "ようこそ！",
                        description: `${username}さん、アカウント登録ありがとうございます！`,
                        duration: 5000,
                    });
                    // Cookieを削除（オプション）
                    document.cookie = "firstLogin=; max-age=0; path=/;";
                }
            } catch (error) {
                console.error('Failed to parse firstLogin cookie:', error);
            }
        }
    }, [toast]);

    return null;  // このコンポーネントは表示要素を持ちません
};

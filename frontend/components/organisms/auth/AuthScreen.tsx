"use client";
import { useState } from "react";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Alert, AlertDescription } from "@/components/ui/alert";
import LoginForm from "@/components/organisms/auth/LoginForm";
import RegisterForm from "@/components/organisms/auth/RegisterForm";

const AuthScreen: React.FC = () => {
    const [activeTab, setActiveTab] = useState("login");
    const [generalError, setGeneralError] = useState<string | null>(null);

    const handleError = (error: Error) => {
        setGeneralError(error.message);
    };

    return (
        <div className="min-h-screen flex items-center justify-center p-4">
            <Card className="w-full max-w-md backdrop-blur-sm">
                <CardHeader>
                    <CardTitle className="text-2xl font-bold text-center">
                        DevTrackr
                    </CardTitle>
                    <CardDescription className="text-center text-text-secondary">ログインまたはアカウント登録をしてください</CardDescription>
                </CardHeader>
                <CardContent>
                    <Tabs value={activeTab} onValueChange={setActiveTab}>
                        <TabsList className="grid w-full grid-cols-2">
                            <TabsTrigger value="login">ログイン</TabsTrigger>
                            <TabsTrigger value="register">アカウント登録</TabsTrigger>
                        </TabsList>
                        <TabsContent value="login">
                            <LoginForm onError={handleError} />
                        </TabsContent>
                        <TabsContent value="register">
                            <RegisterForm onError={handleError} />
                        </TabsContent>
                    </Tabs>
                    {generalError && (
                        <Alert variant="destructive" className="mt-4">
                            <AlertDescription>{generalError}</AlertDescription>
                        </Alert>
                    )}
                </CardContent>
                <CardFooter className="flex justify-center">
                    <p className="text-sm text-text-secondary">
                        &copy; 2024 エンジニア向けダッシュボード All rights reserved.
                    </p>
                </CardFooter>
            </Card>
        </div>
    )
};

export default AuthScreen;
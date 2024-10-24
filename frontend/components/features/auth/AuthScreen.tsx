import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { TabsContent } from "@/components/ui/tabs";
import LoginForm from "@/components/features/auth/LoginForm";
import RegisterForm from "@/components/features/auth/RegisterForm";
import AuthTabs from "@/components/features/auth/AuthTabs";

const AuthScreen = () => {
    return (
        <div className="min-h-screen flex items-center justify-center p-4">
            <Card className="w-full max-w-md backdrop-blur-sm">
                <CardHeader>
                    <CardTitle className="text-2xl font-bold text-center text-primary">DevTrackr</CardTitle>
                    <CardDescription className="text-center text-text-secondary">ログインまたはアカウント登録をしてください</CardDescription>
                </CardHeader>
                <CardContent>
                    <AuthTabs>
                        <TabsContent value="login">
                            <LoginForm />
                        </TabsContent>
                        <TabsContent value="register">
                            <RegisterForm />
                        </TabsContent>
                    </AuthTabs>
                </CardContent>
                <CardFooter className="flex justify-center">
                    <p className="text-sm text-text-secondary">
                        &copy; 2024 Engineer Dashboard All rights reserved.
                    </p>
                </CardFooter>
            </Card>
        </div>
    );
};

export default AuthScreen;
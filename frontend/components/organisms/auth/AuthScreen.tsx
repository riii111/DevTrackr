import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { TabsContent } from "@/components/ui/tabs";
import LoginForm from "@/components/organisms/auth/LoginForm";
import RegisterForm from "@/components/organisms/auth/RegisterForm";
import ClientTabs from "@/components/organisms/auth/ClientTabs";
import ClientAlert from "@/components/organisms/auth/ClientAlert";

const AuthScreen = () => {
    return (
        <div className="min-h-screen bg-white bg-opacity-30 flex items-center justify-center p-4">
            <Card className="w-full max-w-md bg-white-34 backdrop-blur-sm">
                <CardHeader>
                    <CardTitle className="text-2xl font-bold text-center text-black">DevTrackr</CardTitle>
                    <CardDescription className="text-center text-text-secondary">ログインまたはアカウント登録をしてください</CardDescription>
                </CardHeader>
                <CardContent>
                    <ClientTabs>
                        <TabsContent value="login">
                            <LoginForm />
                        </TabsContent>
                        <TabsContent value="register">
                            <RegisterForm />
                        </TabsContent>
                    </ClientTabs>
                    <ClientAlert />
                </CardContent>
                <CardFooter className="flex justify-center">
                    <p className="text-sm text-text-secondary">
                        &copy; 2024 エンジニア向けダッシュボード All rights reserved.
                    </p>
                </CardFooter>
            </Card>
        </div>
    );
};

export default AuthScreen;
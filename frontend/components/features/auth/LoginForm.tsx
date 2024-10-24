"use client";
import { useState } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import FormField from "@/components/core/FormField";
import { loginAction, LoginActionResult } from "@/lib/actions/auth";

const loginSchema = z.object({
    email: z.string().email("有効なメールアドレスを入力してください"),
    password: z.string().min(8, "パスワードは8文字以上である必要があります"),
});

type LoginFormData = z.infer<typeof loginSchema>;

const LoginForm: React.FC = () => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);
        setError(null);

        const formData = new FormData(event.currentTarget);
        const rawData = {
            email: formData.get("email") as string,
            password: formData.get("password") as string,
        };

        try {
            const validatedData = loginSchema.parse(rawData);
            const result = await loginAction(
                validatedData.email,
                validatedData.password
            );

            // リダイレクトの場合はresultが一瞬undefinedになる
            if (result && !result.success) {
                setError(result.error || "ログインに失敗しました。");
            }
        } catch (error) {
            if (error instanceof z.ZodError) {
                setError(error.errors[0].message);
            } else {
                setError("予期せぬエラーが発生しました。");
            }
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <form onSubmit={handleSubmit} noValidate>
            {error && <div className="text-red-500 mb-4">{error}</div>}
            <div className="space-y-4">
                <FormField
                    id="email"
                    name="email"
                    type="email"
                    label="メールアドレス"
                    placeholder="your@email.com"
                    required={true}
                />
                <FormField
                    id="password"
                    name="password"
                    type="password"
                    label="パスワード"
                    required={true}
                />
                <Button
                    type="submit"
                    className="w-full hover:text-accent"
                    disabled={isLoading}
                >
                    {isLoading ? "ログイン中..." : "ログイン"}
                </Button>
            </div>
        </form>
    );
};

export default LoginForm;

"use client";
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { z } from 'zod';
import { Button } from "@/components/ui/button";
import FormField from '@/components/molecules/FormField';
import { useAuthApi } from '@/lib/hooks/useAuthApi';

const loginSchema = z.object({
    email: z.string().email('有効なメールアドレスを入力してください'),
    password: z.string().min(8, 'パスワードは8文字以上である必要があります'),
});

type LoginFormData = z.infer<typeof loginSchema>;

const LoginForm: React.FC = () => {
    const [isLoading, setIsLoading] = useState(false);
    const router = useRouter();
    const { login } = useAuthApi();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);

        const formData = new FormData(event.currentTarget);
        const rawData = {
            email: formData.get('email') as string,
            password: formData.get('password') as string,
        };

        try {
            const validatedData = loginSchema.parse(rawData);
            await loginUser(validatedData);
            router.push('/dashboard');
        } catch (error) {
            if (error instanceof z.ZodError) {
                // フォームバリデーションエラーの処理
                error.errors.forEach(err => {
                    const field = document.getElementById(err.path[0] as string);
                    if (field instanceof HTMLInputElement) {
                        field.setCustomValidity(err.message);
                        field.reportValidity();
                    }
                });
            } else if (error instanceof Error) {
                window.dispatchEvent(new CustomEvent('authError', { detail: error }));
            }
        } finally {
            setIsLoading(false);
        }
    };

    const loginUser = async (data: LoginFormData) => {
        try {
            await login(data.email, data.password);
        } catch (error) {
            throw new Error('ログインに失敗しました');
        }
    };

    return (
        <form onSubmit={handleSubmit} noValidate>
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
                <Button type="submit" className="w-full bg-text-primary text-white hover:bg-secondary hover:text-accent" disabled={isLoading}>
                    {isLoading ? 'ログイン中...' : 'ログイン'}
                </Button>
            </div>
        </form>
    );
};

export default LoginForm;
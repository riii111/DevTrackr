"use client";
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { z } from 'zod';
import { Button } from "@/components/ui/button";
import FormField from '@/components/molecules/FormField';

const registerSchema = z.object({
    name: z.string().min(1, '名前を入力してください'),
    email: z.string().email('有効なメールアドレスを入力してください'),
    password: z.string().min(8, 'パスワードは8文字以上である必要があります'),
});

type RegisterFormData = z.infer<typeof registerSchema>;


const RegisterForm: React.FC = () => {
    const [isLoading, setIsLoading] = useState(false);
    const router = useRouter();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);

        const formData = new FormData(event.currentTarget);
        const rawData = {
            name: formData.get('name') as string,
            email: formData.get('email') as string,
            password: formData.get('password') as string,
        };

        try {
            const validatedData = registerSchema.parse(rawData);
            await registerUser(validatedData);
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

    // TODO: APIの共通ロジック作って置き換える（別issue）
    const registerUser = async (data: RegisterFormData) => {
        const response = await fetch('https://your-api-url.com/register', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data),
        });
        if (!response.ok) throw new Error('アカウント登録に失敗しました');
        const responseData = await response.json();
        localStorage.setItem('token', responseData.token);
    };

    return (
        <form onSubmit={handleSubmit} noValidate>
            <div className="space-y-4">
                <FormField
                    id="name"
                    name="name"
                    type="text"
                    label="名前"
                    placeholder="山田 太郎"
                    required={true}
                />
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
                <Button type="submit" className="w-full bg-black text-white" disabled={isLoading}>
                    {isLoading ? 'アカウント登録中...' : 'アカウント登録'}
                </Button>
            </div>
        </form>
    );
};

export default RegisterForm;
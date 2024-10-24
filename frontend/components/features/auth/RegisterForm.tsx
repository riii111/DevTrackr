"use client";
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { z } from 'zod';
import { Button } from "@/components/ui/button";
import FormField from '@/components/core/FormField';
import { registerAction, RegisterActionResult } from '@/lib/actions/auth';

const registerSchema = z.object({
    name: z.string().min(1, '名前を入力してください'),
    email: z.string().email('有効なメールアドレスを入力してください'),
    password: z.string().min(8, 'パスワードは8文字以上である必要があります'),
});

type RegisterFormData = z.infer<typeof registerSchema>;

const RegisterForm: React.FC = () => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const router = useRouter();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);
        setError(null);

        const formData = new FormData(event.currentTarget);
        const rawData = {
            name: formData.get('name') as string,
            email: formData.get('email') as string,
            password: formData.get('password') as string,
        };

        try {
            const validatedData = registerSchema.parse(rawData);
            const result: RegisterActionResult = await registerAction(
                validatedData.name,
                validatedData.email,
                validatedData.password
            );

            if (!result.success) {
                setError(result.error || "アカウント登録に失敗しました。");
            }
            // 成功時はサーバーサイドでリダイレクトされるため、ここでは何もしない
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
                <Button type="submit" className="w-full hover:bg-secondary hover:text-accent" disabled={isLoading}>
                    {isLoading ? 'アカウント登録中...' : 'アカウント登録'}
                </Button>
            </div>
        </form>
    );
};

export default RegisterForm;

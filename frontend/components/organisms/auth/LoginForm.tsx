import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { z } from 'zod';
import { Button } from "@/components/ui/button";
import FormField from '@/components/molecules/FormField';

const loginSchema = z.object({
    email: z.string().email('有効なメールアドレスを入力してください'),
    password: z.string().min(8, 'パスワードは8文字以上である必要があります'),
});

type LoginFormData = z.infer<typeof loginSchema>;

interface LoginFormProps {
    onError: (error: Error) => void;
}

const LoginForm: React.FC<LoginFormProps> = ({ onError }) => {
    const [isLoading, setIsLoading] = useState(false);
    const router = useRouter();

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
                onError(error);
            }
        } finally {
            setIsLoading(false);
        }
    };

    // TODO: APIの共通ロジック作って置き換える（別issue）
    const loginUser = async (data: LoginFormData) => {
        const response = await fetch('https://your-api-url.com/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data),
        });
        if (!response.ok) throw new Error('ログインに失敗しました');
        const responseData = await response.json();
        localStorage.setItem('token', responseData.token);
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
                <Button type="submit" className="w-full bg-black text-white" disabled={isLoading}>
                    {isLoading ? 'ログイン中...' : 'ログイン'}
                </Button>
            </div>
        </form>
    );
};

export default LoginForm;
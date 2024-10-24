"use client";
import { useState } from 'react';
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
    const [formData, setFormData] = useState<RegisterFormData>({ name: '', email: '', password: '' });
    const [errors, setErrors] = useState<Partial<Record<keyof RegisterFormData | 'form', string>>>({});
    const [isFormValid, setIsFormValid] = useState(false);

    const validateForm = (data: RegisterFormData) => {
        try {
            registerSchema.parse(data);
            setIsFormValid(true);
        } catch (error) {
            setIsFormValid(false);
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        const newFormData = { ...formData, [name]: value };
        setFormData(newFormData);
        validateForm(newFormData);
    };

    const handleInputBlur = (e: React.FocusEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        try {
            if (name === 'name' || name === 'email' || name === 'password') {
                registerSchema.shape[name].parse(value);
            }
            setErrors(prev => ({ ...prev, [name]: undefined }));
        } catch (error) {
            if (error instanceof z.ZodError) {
                setErrors(prev => ({ ...prev, [name]: error.errors[0].message }));
            }
        }
        validateForm(formData);
    };

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);
        setErrors({});

        try {
            const validatedData = registerSchema.parse(formData);
            const result: RegisterActionResult = await registerAction(
                validatedData.name,
                validatedData.email,
                validatedData.password
            );

            if (result && !result.success) {
                setErrors(prev => ({ ...prev, form: result.error || "アカウント登録に失敗しました。" }));
            }
        } catch (error) {
            if (error instanceof z.ZodError) {
                const fieldErrors: Partial<Record<keyof RegisterFormData, string>> = {};
                error.errors.forEach(err => {
                    if (err.path[0]) {
                        fieldErrors[err.path[0] as keyof RegisterFormData] = err.message;
                    }
                });
                setErrors(prev => ({ ...prev, ...fieldErrors }));
            } else {
                setErrors(prev => ({ ...prev, form: "予期せぬエラーが発生しました。" }));
            }
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <form onSubmit={handleSubmit} noValidate>
            {errors.form && <div className="text-red-500 mb-4">{errors.form}</div>}
            <div className="space-y-4">
                <FormField
                    id="name"
                    name="name"
                    type="text"
                    label="名前"
                    placeholder="山田 太郎"
                    required={true}
                    value={formData.name}
                    onChange={handleInputChange}
                    onBlur={handleInputBlur}
                    error={errors.name}
                />
                <FormField
                    id="email"
                    name="email"
                    type="email"
                    label="メールアドレス"
                    placeholder="your@email.com"
                    required={true}
                    value={formData.email}
                    onChange={handleInputChange}
                    onBlur={handleInputBlur}
                    error={errors.email}
                />
                <FormField
                    id="password"
                    name="password"
                    type="password"
                    label="パスワード"
                    required={true}
                    value={formData.password}
                    onChange={handleInputChange}
                    onBlur={handleInputBlur}
                    error={errors.password}
                />
                <Button
                    type="submit"
                    className="w-full hover:bg-secondary hover:text-accent"
                    disabled={isLoading || !isFormValid}
                >
                    {isLoading ? '登録処理中...' : 'アカウント登録'}
                </Button>
            </div>
        </form>
    );
};

export default RegisterForm;

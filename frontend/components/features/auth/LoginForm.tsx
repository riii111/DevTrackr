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
    const [formData, setFormData] = useState<LoginFormData>({ email: '', password: '' });
    const [errors, setErrors] = useState<Partial<Record<keyof LoginFormData | 'form', string>>>({});
    const [isFormValid, setIsFormValid] = useState(false);

    const validateForm = (data: LoginFormData) => {
        try {
            loginSchema.parse(data);
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
            if (name === 'email' || name === 'password') {
                loginSchema.shape[name].parse(value);
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
            const validatedData = loginSchema.parse(formData);
            const result: LoginActionResult = await loginAction(
                validatedData.email,
                validatedData.password
            );

            // リダイレクトの場合はresultが一瞬undefinedになる
            if (result && !result.success) {
                setErrors(prev => ({ ...prev, form: result.error || "ログインに失敗しました。" }));
            }
        } catch (error) {
            if (error instanceof z.ZodError) {
                const fieldErrors: Partial<Record<keyof LoginFormData, string>> = {};
                error.errors.forEach(err => {
                    if (err.path[0]) {
                        fieldErrors[err.path[0] as keyof LoginFormData] = err.message;
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
                    className="w-full hover:text-accent"
                    disabled={isLoading || !isFormValid}
                >
                    {isLoading ? "ログイン中..." : "ログイン"}
                </Button>
            </div>
        </form>
    );
};

export default LoginForm;

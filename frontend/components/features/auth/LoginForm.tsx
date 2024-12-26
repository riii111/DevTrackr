"use client";
import { useForm } from "@conform-to/react";
import { parseWithZod } from "@conform-to/zod";
import { useTransition } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import FormField from "@/components/core/FormField";
import { loginAction } from "@/lib/actions/auth";

const loginSchema = z.object({
    email: z.string({
        required_error: "メールアドレスを入力してください",
    }).email("有効なメールアドレスを入力してください"),
    password: z.string({
        required_error: "パスワードを入力してください",
    }).min(8, "パスワードは8文字以上である必要があります"),
});

const LoginForm: React.FC = () => {
    const [isPending, startTransition] = useTransition();
    const [form, { email, password }] = useForm({
        id: "login-form",
        defaultValue: {
            email: "",
            password: ""
        },
        onValidate: ({ formData }) => {
            return parseWithZod(formData, {
                schema: loginSchema
            });
        },
        shouldValidate: "onBlur",
        shouldRevalidate: "onInput",
        onSubmit: async (event: React.FormEvent<HTMLFormElement>) => {
            event.preventDefault();// フォームのデフォルト送信操作とServer Actionの実行が競合するのを防ぐ

            const formData = new FormData(event.currentTarget);
            const submission = parseWithZod(formData, {
                schema: loginSchema
            });

            if (submission.status !== "success") {
                return submission.reply();
            }

            startTransition(async () => {
                try {
                    await loginAction(
                        submission.value.email,
                        submission.value.password
                    );
                } catch (error) {
                    if (error instanceof Error && !error.message.includes('NEXT_REDIRECT')) {
                        submission.reply({
                            formErrors: ["予期せぬエラーが発生しました。"]
                        });
                    }
                }
            });
        }
    });

    const isSubmitDisabled =
        isPending ||
        !form.dirty ||
        form.status === 'error' ||
        Object.values({ email, password }).some(field => field.errors);

    return (
        <form
            id={form.id}
            onSubmit={form.onSubmit}
            noValidate
        >
            {form.errors && (
                <div className="text-red-500 mb-4">
                    {form.errors.map((error, i) => (
                        <div key={i}>{error}</div>
                    ))}
                </div>
            )}
            <div className="space-y-4">
                <FormField
                    id={email.id}
                    name={email.name}
                    type="email"
                    label="メールアドレス"
                    placeholder="your@email.com"
                    required={true}
                    error={email.errors?.[0]}
                />
                <FormField
                    id={password.id}
                    name={password.name}
                    type="password"
                    label="パスワード"
                    required={true}
                    error={password.errors?.[0]}
                />
                <Button
                    type="submit"
                    className="w-full hover:text-accent"
                    disabled={isSubmitDisabled}
                >
                    {isPending ? "ログイン中..." : "ログイン"}
                </Button>
            </div>
        </form>
    );
};

export default LoginForm;

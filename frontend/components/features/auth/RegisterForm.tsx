"use client";
import { useForm } from "@conform-to/react";
import { parseWithZod } from "@conform-to/zod";
import { useTransition } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import FormField from "@/components/core/FormField";
import { registerAction } from "@/lib/actions/auth";

const registerSchema = z.object({
    name: z.string({
        required_error: "名前を入力してください",
    }).min(1, "名前を入力してください"),
    email: z.string({
        required_error: "メールアドレスを入力してください",
    }).email("有効なメールアドレスを入力してください"),
    password: z.string({
        required_error: "パスワードを入力してください",
    }).min(8, "パスワードは8文字以上である必要があります"),
});

const RegisterForm: React.FC = () => {
    const [isPending, startTransition] = useTransition();
    const [form, { name, email, password }] = useForm({
        id: "register-form",
        defaultValue: {
            name: "",
            email: "",
            password: ""
        },
        onValidate: ({ formData }) => {
            return parseWithZod(formData, {
                schema: registerSchema
            });
        },
        shouldValidate: "onBlur",
        shouldRevalidate: "onInput",
        onSubmit: async (event: React.FormEvent<HTMLFormElement>) => {
            event.preventDefault(); // フォームのデフォルト送信操作とServer Actionの実行が競合するのを防ぐ

            const formData = new FormData(event.currentTarget);
            const submission = parseWithZod(formData, {
                schema: registerSchema
            });

            if (submission.status !== "success") {
                return submission.reply();
            }

            startTransition(async () => {
                try {
                    await registerAction(
                        submission.value.name,
                        submission.value.email,
                        submission.value.password
                    );
                } catch (error) {
                    if (error instanceof Error && !error.message.includes('NEXT_REDIRECT')) {
                        submission.reply({
                            formErrors: [error.message || "アカウント登録に失敗しました。"]
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
        Object.values({ name, email, password }).some(field => field.errors);

    return (
        <form id={form.id} onSubmit={form.onSubmit} noValidate>
            {form.errors && (
                <div className="text-red-500 mb-4">
                    {form.errors.map((error, i) => (
                        <div key={i}>{error}</div>
                    ))}
                </div>
            )}
            <div className="space-y-4">
                <FormField
                    id={name.id}
                    name={name.name}
                    type="text"
                    label="名前"
                    placeholder="山田 太郎"
                    required={true}
                    error={name.errors?.[0]}
                />
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
                    className="w-full hover:bg-secondary hover:text-accent"
                    disabled={isSubmitDisabled}
                >
                    {isPending ? "登録処理中..." : "アカウント登録"}
                </Button>
            </div>
        </form>
    );
};

export default RegisterForm;

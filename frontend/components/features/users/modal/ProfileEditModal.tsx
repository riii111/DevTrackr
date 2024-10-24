"use client";

import React, { useState, useCallback, useEffect, useMemo } from 'react';
import { useRouter } from 'next/navigation';
import { Button } from "@/components/ui/button"
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "@/components/ui/card"
import { User, UpdateUserRequest } from '@/types/user';
import ProfileEditForm, { profileSchema, ProfileFormData } from '@/components/features/users/modal/content/ProfileEditForm';
import { z } from 'zod';
import { useToast } from "@/lib/hooks/use-toast";
import { ApiError } from '@/lib/api/core';
import { updateUserAction } from '@/lib/actions/user';

interface ProfileEditProps {
    initialUser: User;
}

const MemoizedProfileEditForm = React.memo(ProfileEditForm);

export default function ProfileEditModal({ initialUser }: ProfileEditProps) {
    const [user, setUser] = useState<ProfileFormData>({
        username: initialUser.username,
        email: initialUser.email,
        role: initialUser.role,
        avatar: initialUser.avatar_url
    });
    const [errors, setErrors] = useState<Partial<Record<keyof ProfileFormData, string>>>({});
    const [avatarPreview, setAvatarPreview] = useState<string | null>(null);
    const [avatarFile, setAvatarFile] = useState<File | null>(null);
    const router = useRouter();
    const { toast } = useToast();

    useEffect(() => {
        if (initialUser.avatar_url) {
            const publicUrl = initialUser.avatar_url.replace(
                'minio:9000',
                process.env.NEXT_PUBLIC_MINIO_PUBLIC_URL?.replace('http://', '') || 'localhost:9000'
            );
            setAvatarPreview(publicUrl);
        }
    }, [initialUser.avatar_url]);

    const handleClose = useCallback(() => {
        router.back();
    }, [router]);

    const validateField = useCallback((name: keyof ProfileFormData, value: any) => {
        try {
            if (name === 'avatar') {
                if (value.size > 2 * 1024 * 1024) {
                    throw new Error('ファイルサイズは2MB以下にしてください。');
                }
                if (!['image/jpeg', 'image/png', 'image/gif'].includes(value.type)) {
                    throw new Error('JPG、PNG、GIF形式の画像のみ許可されています。');
                }
            } else {
                profileSchema.shape[name].parse(value);
            }
            setErrors(prev => ({ ...prev, [name]: undefined }));
        } catch (error) {
            if (error instanceof z.ZodError) {
                setErrors(prev => ({ ...prev, [name]: error.errors[0]?.message }));
            } else if (error instanceof Error) {
                setErrors(prev => ({ ...prev, [name]: error.message }));
            }
        }
    }, []);

    const fileToBase64 = useCallback((file: File): Promise<string> => {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.readAsDataURL(file);
            reader.onload = () => {
                const result = reader.result as string;
                resolve(result);
            };
            reader.onerror = error => reject(error);
        });
    }, []);

    const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setUser(prev => ({ ...prev, [name]: value }));
    }, []);

    const handleInputBlur = useCallback((e: React.FocusEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        validateField(name as keyof ProfileFormData, value);
    }, [validateField]);

    const handleRoleChange = useCallback((value: string) => {
        setUser(prev => ({ ...prev, role: value as User['role'] }));
        validateField('role', value);
    }, [validateField]);

    const handleAvatarChange = useCallback((file: File) => {
        setAvatarFile(file);
        const reader = new FileReader();
        reader.onloadend = () => {
            setAvatarPreview(reader.result as string);
        };
        reader.readAsDataURL(file);
        validateField('avatar', file);
    }, [validateField]);

    const handleSubmit = useCallback(async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const validatedData = profileSchema.parse(user);
            const updateData: UpdateUserRequest = {
                username: validatedData.username,
                email: validatedData.email,
            };
            if (validatedData.role) {
                updateData.role = validatedData.role;
            }
            if (avatarFile) {
                // ここでBase64エンコードした画像データを送信
                const base64 = await fileToBase64(avatarFile);
                updateData.avatar = base64;
            }

            const result = await updateUserAction(updateData);
            if (result.success) {
                toast({
                    title: 'プロフィールを更新しました',
                    variant: 'success',
                });
                handleClose();
            } else {
                throw new Error(result.error);
            }
        } catch (error) {
            if (error instanceof z.ZodError) {
                // ZodErrorの場合、エラーメッセージを新しいエラーオブジェクトに変換
                const newErrors = error.errors.reduce((acc, curr) => {
                    // 各エラーメッセージを対応するフィールドにマッピング
                    acc[curr.path[0] as keyof ProfileFormData] = curr.message;
                    return acc;
                }, {} as Partial<Record<keyof ProfileFormData, string>>);
                setErrors(newErrors);
                toast({
                    title: 'プロフィールの更新に失敗しました',
                    description: 'フォームの入力内容を確認してください。',
                    variant: 'error',
                });
            } else if (error instanceof ApiError) {
                toast({
                    title: 'プロフィールの更新に失敗しました',
                    description: error.message,
                    variant: 'error',
                });
            } else {
                console.error('Unexpected error:', error);
                toast({
                    title: 'プロフィールの更新に失敗しました',
                    description: '予期せぬエラーが発生しました。もう一度お試しください。',
                    variant: 'error',
                });
            }
        }
    }, [user, avatarFile, toast, handleClose, fileToBase64]);

    const handleOverlayClick = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
        if (e.target === e.currentTarget) {
            handleClose();
        }
    }, [handleClose]);

    const memoizedUser = useMemo(() => ({ ...user, avatar: avatarPreview || user.avatar }), [user, avatarPreview]);

    return (
        <div className="absolute inset-0 bg-black bg-opacity-50 flex justify-center items-center overflow-auto" onClick={handleOverlayClick}>
            <div className="my-8 bg-white rounded-lg shadow-lg w-full max-w-2xl p-6 relative" onClick={e => e.stopPropagation()}>
                <Card className="w-full">
                    <CardHeader>
                        <CardTitle>プロフィール編集</CardTitle>
                        <CardDescription>あなたの個人情報を更新します</CardDescription>
                    </CardHeader>
                    <form onSubmit={handleSubmit}>
                        <CardContent>
                            <MemoizedProfileEditForm
                                user={memoizedUser}
                                errors={errors}
                                onInputChange={handleInputChange}
                                onInputBlur={handleInputBlur}
                                onRoleChange={handleRoleChange}
                                onAvatarChange={handleAvatarChange}
                            />
                        </CardContent>
                        <CardFooter className="flex justify-end space-x-2">
                            <Button variant="outline" type="button" onClick={handleClose}>キャンセル</Button>
                            <Button type="submit">保存</Button>
                        </CardFooter>
                    </form>
                </Card>
            </div>
        </div>
    );
}

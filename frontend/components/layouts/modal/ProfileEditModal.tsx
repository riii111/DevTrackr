"use client";

import React, { useState, useCallback } from 'react';
import { useRouter } from 'next/navigation';
import { Button } from "@/components/ui/button"
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "@/components/ui/card"
import { User } from '@/types/user';
import ProfileEditForm, { profileSchema, ProfileFormData } from '@/components/layouts/modal/content/ProfileEditForm';
import { z } from 'zod';
import { useUserApi } from '@/lib/hooks/useUserApi';
import { toast } from '@/lib/hooks/use-toast';

interface ProfileEditProps {
    initialUser: User;
}

export default function ProfileEditModal({ initialUser }: ProfileEditProps) {
    const [user, setUser] = useState<ProfileFormData>({
        username: initialUser.username,
        email: initialUser.email,
        role: initialUser.role,
        avatar: initialUser.avatar
    });
    const [errors, setErrors] = useState<Partial<Record<keyof ProfileFormData, string>>>({});
    const router = useRouter();
    const { updateUser } = useUserApi();

    const handleClose = useCallback(() => {
        router.back();
    }, [router]);

    const validateField = (name: keyof ProfileFormData, value: string) => {
        try {
            profileSchema.shape[name].parse(value);
            setErrors(prev => ({ ...prev, [name]: undefined }));
        } catch (error) {
            if (error instanceof z.ZodError) {
                setErrors(prev => ({ ...prev, [name]: error.errors[0]?.message }));
            }
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setUser(prev => ({ ...prev, [name]: value }));
    };

    const handleInputBlur = (e: React.FocusEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        validateField(name as keyof ProfileFormData, value);
    };

    const handleRoleChange = (value: string) => {
        setUser(prev => ({ ...prev, role: value as User['role'] }));
        validateField('role', value);
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const validatedData = profileSchema.parse(user);
            updateUser(validatedData);
            toast({
                title: 'プロフィールを更新しました',
                variant: 'default',
            });
            // TODO: UserMenuをrevalidateする
            handleClose();
        } catch (error) {
            if (error instanceof z.ZodError) {
                // ZodErrorの場合、エラーメッセージを新しいエラーオブジェクトに変換
                const newErrors = error.errors.reduce((acc, curr) => {
                    // 各エラーメッセージを対応するフィールドにマッピング
                    acc[curr.path[0] as keyof ProfileFormData] = curr.message;
                    return acc;
                }, {} as Partial<Record<keyof ProfileFormData, string>>);
                setErrors(newErrors);
            }
        }
    };

    const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement>) => {
        if (e.target === e.currentTarget) {
            handleClose();
        }
    };

    return (
        <div className="absolute inset-0 bg-black bg-opacity-50 flex justify-center items-center overflow-auto" onClick={handleOverlayClick}>
            <div className="my-8 bg-white rounded-lg shadow-lg w-full max-w-2xl p-6 relative" onClick={e => e.stopPropagation()}>
                <Card className="w-full">
                    <CardHeader>
                        <CardTitle>プロフィール編集</CardTitle>
                        <CardDescription>あなたの個人情報を更新します</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <ProfileEditForm
                            user={user}
                            errors={errors}
                            onInputChange={handleInputChange}
                            onInputBlur={handleInputBlur}
                            onRoleChange={handleRoleChange}
                            onSubmit={handleSubmit}
                        />
                    </CardContent>
                    <CardFooter className="flex justify-end space-x-2">
                        <Button variant="outline" type="button" onClick={handleClose}>キャンセル</Button>
                        <Button type="submit" onClick={handleSubmit}>保存</Button>
                    </CardFooter>
                </Card>
            </div>
        </div>
    );
}

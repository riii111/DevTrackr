"use client";

import React, { useRef } from 'react';
import { z } from 'zod';
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar"
import { UserRole } from '@/types/user';
import FormField from '@/components/molecules/FormField';

export const profileSchema = z.object({
    username: z.string().min(1, '名前を入力してください'),
    email: z.string().email('有効なメールアドレスを入力してください'),
    role: z.nativeEnum(UserRole).nullable().optional(),
    avatar: z.string().optional().nullable(),
});

export type ProfileFormData = z.infer<typeof profileSchema>;

interface ProfileEditFormProps {
    user: ProfileFormData;
    errors: Partial<Record<keyof ProfileFormData, string>>;
    onInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onInputBlur: (e: React.FocusEvent<HTMLInputElement>) => void;
    onRoleChange: (value: string) => void;
    onAvatarChange: (file: File) => void;
    // onAvatarDelete: () => void;
}

export default function ProfileEditForm({
    user,
    errors,
    onInputChange,
    onInputBlur,
    onRoleChange,
    onAvatarChange,
    // onAvatarDelete,
}: ProfileEditFormProps) {
    const fileInputRef = useRef<HTMLInputElement>(null);

    const handleAvatarClick = () => {
        fileInputRef.current?.click();
    };

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (file) {
            onAvatarChange(file);
        }
    };

    return (
        <div className="space-y-6">
            <div className="flex flex-col items-center space-y-4">
                <Avatar className="w-32 h-32">
                    <AvatarImage src={user.avatar || ''} alt="ユーザーアバター" className="object-cover" />
                    <AvatarFallback>UN</AvatarFallback>
                </Avatar>
                <input
                    type="file"
                    ref={fileInputRef}
                    onChange={handleFileChange}
                    accept="image/*"
                    style={{ display: 'none' }}
                />
                <Button type="button" variant="outline" onClick={handleAvatarClick}>画像を変更</Button>
                {/* TODO: PUT→PATCH変更時に追加 */}
                {/* <div className="flex space-x-2">
                    <Button type="button" variant="outline" onClick={handleAvatarClick}>画像を変更</Button>
                    {user.avatar && (
                        <Button type="button" variant="outline" onClick={onAvatarDelete}>画像を削除</Button>
                    )}
                </div>　*/}
                {errors.avatar && <p className="text-red-500 text-sm">{errors.avatar}</p>}
            </div>

            <FormField
                id="username"
                name="username"
                type="text"
                label="ユーザー名"
                placeholder="ユーザー名を入力"
                value={user.username}
                onChange={onInputChange}
                onBlur={onInputBlur}
                error={errors.username}
                required
            />

            <FormField
                id="email"
                name="email"
                type="email"
                label="メールアドレス"
                placeholder="メールアドレスを入力"
                value={user.email}
                onChange={onInputChange}
                onBlur={onInputBlur}
                error={errors.email}
                required
            />

            <div className="space-y-2">
                <Label htmlFor="role">ロール</Label>
                <Select value={user.role || undefined} onValueChange={onRoleChange}>
                    <SelectTrigger>
                        <SelectValue placeholder="ロールを選択" />
                    </SelectTrigger>
                    <SelectContent>
                        {Object.entries(UserRole).map(([key, value]) => (
                            <SelectItem key={key} value={value}>
                                {value}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
                {errors.role && <p className="text-red-500 text-sm">{errors.role}</p>}
            </div>

            <div className="space-y-2">
                <Button variant="outline" className="w-full">パスワードを変更</Button>
            </div>
        </div>
    );
}

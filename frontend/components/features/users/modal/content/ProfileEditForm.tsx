"use client";

import React, { useRef, useCallback, memo } from 'react';
import { z } from 'zod';
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar"
import { UserRole } from '@/types/user';
import FormField from '@/components/core/FormField';

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

const AvatarSection = memo(({ avatar, onAvatarChange, error }: {
    avatar: string | null,
    onAvatarChange: (file: File) => void,
    error?: string
}) => {
    const fileInputRef = useRef<HTMLInputElement>(null);

    const handleAvatarClick = useCallback(() => {
        fileInputRef.current?.click();
    }, []);

    const handleFileChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (file) {
            onAvatarChange(file);
        }
    }, [onAvatarChange]);

    return (
        <div className="flex flex-col items-center space-y-4">
            <Avatar className="w-32 h-32">
                <AvatarImage src={avatar || ''} alt="ユーザーアバター" className="object-cover" />
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
            {error && <p className="text-red-500 text-sm">{error}</p>}
        </div>
    );
});
AvatarSection.displayName = 'AvatarSection';

const RoleSelect = memo(({ role, onRoleChange, error }: {
    role: string | null | undefined,
    onRoleChange: (value: string) => void,
    error?: string
}) => (
    <div className="space-y-2">
        <Label htmlFor="role">ロール</Label>
        <Select value={role || undefined} onValueChange={onRoleChange}>
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
        {error && <p className="text-red-500 text-sm">{error}</p>}
    </div>
));
RoleSelect.displayName = 'RoleSelect';

export default function ProfileEditForm({
    user,
    errors,
    onInputChange,
    onInputBlur,
    onRoleChange,
    onAvatarChange,
}: ProfileEditFormProps) {
    return (
        <div className="space-y-6">
            <AvatarSection
                avatar={user.avatar || ''}
                onAvatarChange={onAvatarChange}
                error={errors.avatar}
            />

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

            <RoleSelect
                role={user.role}
                onRoleChange={onRoleChange}
                error={errors.role}
            />
        </div>
    );
}

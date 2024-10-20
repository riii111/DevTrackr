import React from 'react';
import { Badge } from "@/components/ui/badge";
import { CompanyStatus } from "@/types/company";
import { ProjectStatus } from "@/types/project";

interface StatusColors {
    [key: string]: string;
}

const statusColors: StatusColors = {
    PendingContract: "bg-yellow-100 text-yellow-800",
    Contract: "bg-green-100 text-green-800",
    Completed: "bg-blue-100 text-blue-800",
    Cancelled: "bg-red-100 text-red-800",
    Planning: "bg-blue-100 text-blue-800",
    InProgress: "bg-yellow-100 text-yellow-800",
    OnHold: "bg-gray-100 text-gray-800",
};

const statusLabels: StatusColors = {
    PendingContract: "契約待ち",
    Contract: "契約中",
    Completed: "完了",
    Cancelled: "キャンセル",
    Planning: "未着手",
    InProgress: "進行中",
    OnHold: "保留",
};

interface StatusBadgeProps {
    status: CompanyStatus | ProjectStatus;
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({ status }) => {
    return (
        <Badge className={`${statusColors[status]} px-2 py-1 rounded-full text-xs font-medium`}>
            {statusLabels[status]}
        </Badge>
    );
};


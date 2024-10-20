import { Skeleton } from "@/components/ui/skeleton"

export const LoadingSkeleton: React.FC = () => (
    <div className="space-y-4">
        <Skeleton className="h-8 w-3/4" />
        <Skeleton className="h-4 w-1/2" />
        <Skeleton className="h-4 w-2/3" />
        <Skeleton className="h-4 w-1/3" />
    </div>
);

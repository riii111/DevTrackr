import { Button } from "@/components/ui/button"
import { IconType } from 'react-icons';
import { cn } from "@/lib/utils"

interface AtomsButtonWithIconProps {
    icon?: IconType
    text: string
    btnColor?: string
    iconColor?: string
    iconSize?: number
    textColor?: string
    textBold?: boolean
    rounded?: number
    loading?: boolean
    disabled?: boolean
    onClick?: () => void
    variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link"
    size?: "default" | "sm" | "lg" | "icon"
}

export default function AtomsButtonWithIcon({
    icon: Icon,
    text,
    // btnColor = 'bg-primary',
    btnColor,
    iconColor = 'text-text-secondary',
    textColor = 'text-text-primary',
    iconSize = 20,
    textBold = false,
    rounded = 8,
    loading = false,
    disabled = false,
    onClick,
    variant = "default",
    size = "default"
}: AtomsButtonWithIconProps) {
    const buttonClasses = cn(
        "flex items-center justify-center",
        btnColor,
        disabled && "opacity-50 cursor-not-allowed",
        loading && "cursor-wait",
        "transition-colors duration-200",
        "group bg-white text-primary hover:bg-secondary hover:text-accent"
    );

    const textClasses = cn(
        textColor,
        textBold && "font-bold"
    );

    return (
        <Button
            className={buttonClasses}
            style={{ borderRadius: `${rounded}px` }}
            disabled={disabled || loading}
            onClick={onClick}
            variant={variant}
            size={size}
        >
            {Icon && <Icon className={cn(iconColor, "mr-2")} size={iconSize} />}
            <span className={textClasses}>{text}</span>
            {loading && <span className="ml-2 text-text-secondary">Loading...</span>}
        </Button>
    );
}
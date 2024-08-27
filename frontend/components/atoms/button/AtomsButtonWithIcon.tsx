import { Button } from '@headlessui/react';
import { IconType } from 'react-icons';

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
}

export default function AtomsButtonWithIcon({
    icon: Icon,
    text,
    btnColor = 'bg-white',
    iconColor = 'text-text-secondary',
    textColor = 'text-text-primary',
    iconSize = 20,
    textBold = false,
    rounded = 8,
    loading = false,
    disabled = false,
    onClick
}: AtomsButtonWithIconProps) {
    const buttonClasses = `
        flex items-center justify-center
        px-4 py-2
        ${btnColor}
        ${disabled ? 'opacity-50 cursor-not-allowed' : ''}
        ${loading ? 'cursor-wait' : ''}
        transition-colors duration-200
        group bg-white text-primary hover:bg-secondary hover:text-accent
    `;

    const textClasses = `
        ${textColor}
        ${textBold ? 'font-bold' : ''}
    `;

    return (
        <Button
            className={buttonClasses}
            style={{ borderRadius: `${rounded}px` }}
            disabled={disabled || loading}
            onClick={onClick}
        >
            {Icon && <Icon className={`${iconColor} mr-2`} size={iconSize} />}
            <span className={textClasses}>{text}</span>
            {loading && <span className="ml-2 text-text-secondary">Loading...</span>}
        </Button>
    );
}
import { Dialog, DialogPanel, DialogTitle } from '@headlessui/react';
import { ReactNode } from 'react';

interface Props {
    isOpen: boolean;
    onClose: () => void;
    title?: string;
    noGutters?: boolean;
    loading?: boolean;
    description?: string;
    children?: ReactNode;
    width?: number;
}

export default function MoleculesDialog({
    isOpen,
    onClose,
    title = undefined,
    noGutters = false,
    loading = false,
    description,
    children,
    width = 640,
}: Props) {
    return (
        <Dialog open={isOpen} onClose={onClose} className="relative z-50">
            <div className="fixed inset-0 bg-black/30" aria-hidden="true" />
            <div className="fixed inset-0 flex items-center justify-center p-4">
                <DialogPanel className={`w-full rounded bg-white ${noGutters ? '' : 'p-6'}`} style={{ maxWidth: `${width}px` }}>
                    {title && <DialogTitle className="text-lg font-medium leading-6 text-gray-900 p-4">{title}</DialogTitle>}
                    {description && <p className="mt-2 text-sm text-gray-500 px-4">{description}</p>}
                    {loading ? (
                        <div className="flex justify-center items-center h-32">
                            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
                        </div>
                    ) : (
                        <div className={noGutters ? '' : 'mt-4'}>
                            {children}
                        </div>
                    )}
                </DialogPanel>
            </div>
        </Dialog>
    );
}
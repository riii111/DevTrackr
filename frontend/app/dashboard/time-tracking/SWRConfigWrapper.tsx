'use client';

import { SWRConfig } from 'swr';

export default function SWRConfigWrapper({ children, fallback }: { children: React.ReactNode, fallback: Record<string, any> }) {
    return (
        <SWRConfig value={{ fallback }}>
            {children}
        </SWRConfig>
    );
}

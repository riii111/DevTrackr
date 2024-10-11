"use client";

import { useEffect } from 'react';
import { useDrawerStore } from '@/lib/store/useDrawerStore';
import { useSearchParams } from 'next/navigation';
import { ProjectDrawer } from '@/components/organisms/projects/ProjectDrawer/content/ProjectDrawer';

export default function ProjectDetails() {
    const { drawerState, handleOpen, handleClose } = useDrawerStore();
    const searchParams = useSearchParams();

    useEffect(() => {
        const projectId = searchParams.get('projectId');
        if (projectId) {
            handleOpen('main', { id: projectId, dataType: 'project' });
        } else {
            handleClose('main');
        }
    }, [searchParams, handleOpen, handleClose]);

    if (!drawerState.main.isOpen) return null;

    return (
        <ProjectDrawer />
    );
}
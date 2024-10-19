import React, { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { PencilIcon, CheckIcon, XIcon } from 'lucide-react';
import { statusColors } from "@/lib/constants/ProjectStatusColors";
import { ProjectStatus } from "@/types/project";

interface ProjectDetailsProps {
    project: any;
    onSave: (updatedProject: any) => void;
}

export const ProjectDetails: React.FC<ProjectDetailsProps> = ({ project, onSave }) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedProject, setEditedProject] = useState(project);

    const handleEdit = () => {
        setIsEditing(true);
    };

    const handleSave = () => {
        onSave(editedProject);
        setIsEditing(false);
    };

    const handleCancel = () => {
        setEditedProject(project);
        setIsEditing(false);
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setEditedProject(prev => ({ ...prev, [name]: value }));
    };

    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-2xl font-bold">
                    {isEditing ? (
                        <Input
                            name="title"
                            value={editedProject.title}
                            onChange={handleInputChange}
                            className="text-2xl font-bold"
                        />
                    ) : (
                        editedProject.title
                    )}
                </CardTitle>
                <Badge className={statusColors[project.status as ProjectStatus]}>
                    {project.status}
                </Badge>
            </CardHeader>
            <CardContent>
                <dl className="space-y-4">
                    <div>
                        <dt className="font-semibold">技術スタック</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="skill_labels"
                                    value={editedProject.skill_labels?.join(', ')}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedProject.skill_labels?.join(', ')
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">内容</dt>
                        <dd>
                            {isEditing ? (
                                <Textarea
                                    name="description"
                                    value={editedProject.description}
                                    onChange={handleInputChange}
                                    rows={3}
                                />
                            ) : (
                                editedProject.description
                            )}
                        </dd>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <dt className="font-semibold">時給</dt>
                            <dd>
                                {isEditing ? (
                                    <Input
                                        name="hourly_pay"
                                        type="number"
                                        value={editedProject.hourly_pay}
                                        onChange={handleInputChange}
                                    />
                                ) : (
                                    `${editedProject.hourly_pay}円`
                                )}
                            </dd>
                        </div>
                        <div>
                            <dt className="font-semibold">総作業時間</dt>
                            <dd>
                                {isEditing ? (
                                    <Input
                                        name="total_working_time"
                                        type="number"
                                        value={editedProject.total_working_time}
                                        onChange={handleInputChange}
                                    />
                                ) : (
                                    `${Math.floor(editedProject.total_working_time / 3600)}時間`
                                )}
                            </dd>
                        </div>
                    </div>
                </dl>
                <div className="mt-4 flex justify-end space-x-2">
                    {isEditing ? (
                        <>
                            <Button onClick={handleCancel} variant="outline" className="text-primary hover:bg-gray-100">
                                <XIcon className="mr-2 h-4 w-4 text-primary" /> キャンセル
                            </Button>
                            <Button onClick={handleSave} variant="default" className="text-white hover:bg-primary/80">
                                <CheckIcon className="mr-2 h-4 w-4 text-white" /> 保存
                            </Button>
                        </>
                    ) : (
                        <Button onClick={handleEdit} variant="outline" className="text-white hover:bg-primary/80 bg-primary">
                            <PencilIcon className="mr-2 h-4 w-4 text-white" /> 編集
                        </Button>
                    )}
                </div>
            </CardContent>
        </Card>
    );
};

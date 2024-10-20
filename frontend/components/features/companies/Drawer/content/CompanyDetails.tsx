import React, { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { PencilIcon, CheckIcon, XIcon } from 'lucide-react';
import { Company, ContractType } from "@/types/company";
import { StatusBadge } from "@/components/core/StatusBadge";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { DatePicker } from "@/components/core/DatePicker";

interface CompanyDetailsProps {
    company: Company;
    onSave: (updatedCompany: Company) => void;
}

const formatMoney = (amount: number) => {
    return amount.toLocaleString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
};


export const CompanyDetails: React.FC<CompanyDetailsProps> = ({ company, onSave }) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedCompany, setEditedCompany] = useState(company);

    const handleEdit = () => {
        setIsEditing(true);
    };

    const handleSave = () => {
        onSave(editedCompany);
        setIsEditing(false);
    };

    const handleCancel = () => {
        setEditedCompany(company);
        setIsEditing(false);
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setEditedCompany((prev) => ({ ...prev, [name]: value }));
    };

    const handleSelectChange = (name: string) => (value: string) => {
        setEditedCompany((prev) => ({ ...prev, [name]: value }));
    };

    const handleDateChange = (name: string) => (date: Date | undefined) => {
        setEditedCompany((prev) => ({ ...prev, [name]: date }));
    };

    const handleAnnualSalesChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setEditedCompany((prev: any) => ({
            ...prev,
            annual_sales: { ...prev.annual_sales, [name]: parseInt(value) }
        }));
    };

    const handleBonusChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setEditedCompany((prev: any) => ({
            ...prev,
            bonus: { ...prev.bonus, [name]: parseInt(value) }
        }));
    };

    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-2xl font-bold">
                    {isEditing ? (
                        <Input
                            name="company_name"
                            value={editedCompany.company_name}
                            onChange={handleInputChange}
                            className="text-2xl font-bold"
                        />
                    ) : (
                        editedCompany.company_name
                    )}
                </CardTitle>
                <StatusBadge status={company.status} />
            </CardHeader>
            <CardContent>
                <dl className="space-y-4">
                    <div>
                        <dt className="font-semibold">設立年</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="establishment_year"
                                    type="number"
                                    value={editedCompany.establishment_year}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedCompany.establishment_year
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">所在地</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="location"
                                    value={editedCompany.location}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedCompany.location
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">Webサイト</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="website_url"
                                    value={editedCompany.website_url}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                <a href={editedCompany.website_url} target="_blank" rel="noopener noreferrer">
                                    {editedCompany.website_url}
                                </a>
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">従業員数</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="employee_count"
                                    type="number"
                                    value={editedCompany.employee_count}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                `${editedCompany.employee_count}人`
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">年間売上（任意）</dt>
                        <dd>
                            {isEditing ? (
                                <>
                                    <Input
                                        name="amount"
                                        type="number"
                                        value={editedCompany.annual_sales?.amount}
                                        onChange={handleAnnualSalesChange}
                                        placeholder="金額"
                                    />
                                    <Input
                                        name="fiscal_year"
                                        type="number"
                                        value={editedCompany.annual_sales?.fiscal_year}
                                        onChange={handleAnnualSalesChange}
                                        placeholder="会計年度"
                                    />
                                </>
                            ) : (
                                editedCompany.annual_sales
                                    ? `${formatMoney(editedCompany.annual_sales.amount)}円（${editedCompany.annual_sales.fiscal_year}年度）`
                                    : '未設定'
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">契約形態</dt>
                        <dd>
                            {isEditing ? (
                                <Select
                                    value={editedCompany.contract_type}
                                    onValueChange={handleSelectChange("contract_type")}
                                >
                                    <SelectTrigger>
                                        <SelectValue placeholder="契約形態を選択" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {Object.values(ContractType).map((type) => (
                                            <SelectItem key={type} value={type}>
                                                {type}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                            ) : (
                                editedCompany.contract_type
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">主要クライアント</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="major_clients"
                                    value={editedCompany.major_clients?.join(', ')}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedCompany.major_clients?.join(', ')
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">主要サービス</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="major_services"
                                    value={editedCompany.major_services?.join(', ')}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedCompany.major_services?.join(', ')
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">平均時給</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="average_hourly_rate"
                                    type="number"
                                    value={editedCompany.average_hourly_rate}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedCompany.average_hourly_rate ? `${editedCompany.average_hourly_rate}円` : '未設定'
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">ボーナス</dt>
                        <dd>
                            {isEditing ? (
                                <div className="flex space-x-2">
                                    <Input
                                        name="amount"
                                        type="number"
                                        value={editedCompany.bonus?.amount}
                                        onChange={handleBonusChange}
                                        placeholder="金額"
                                        className="flex-1"
                                    />
                                    <Input
                                        name="frequency"
                                        type="number"
                                        value={editedCompany.bonus?.frequency}
                                        onChange={handleBonusChange}
                                        placeholder="頻度"
                                        className="flex-1"
                                    />
                                </div>
                            ) : (
                                editedCompany.bonus
                                    ? `${formatMoney(editedCompany.bonus.amount)}円（年${editedCompany.bonus.frequency}回）`
                                    : '未設定'
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">契約開始日</dt>
                        <dd>
                            {isEditing ? (
                                <DatePicker
                                    date={editedCompany.affiliation_start_date ? new Date(editedCompany.affiliation_start_date) : undefined}
                                    setDate={handleDateChange("affiliation_start_date")}
                                />
                            ) : (
                                editedCompany.affiliation_start_date || '未設定'
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">契約終了日（任意）</dt>
                        <dd>
                            {isEditing ? (
                                <DatePicker
                                    date={editedCompany.affiliation_end_date ? new Date(editedCompany.affiliation_end_date) : undefined}
                                    setDate={handleDateChange("affiliation_end_date")}
                                />
                            ) : (
                                editedCompany.affiliation_end_date || '未設定'
                            )}
                        </dd>
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

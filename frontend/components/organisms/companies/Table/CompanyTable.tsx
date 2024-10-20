import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { MdChevronRight } from 'react-icons/md';
import Link from 'next/link';
import { Company, ContractType } from "@/types/company";
import { TruncatedText } from "@/components/atoms/TruncatedText";
import { StatusBadge } from "@/components/atoms/StatusBadge";

const contractTypeLabels: Record<ContractType, string> = {
    FullTime: "正社員",
    PartTime: "パート",
    Contract: "契約",
    Freelance: "フリーランス",
    SideJob: "副業",
};

interface CompanyTableProps {
    companies: Company[];
}

export const CompanyTable: React.FC<CompanyTableProps> = ({ companies }) => {
    if (!companies || companies.length === 0) {
        return <p>企業データがありません。</p>;
    }

    const formatEmployeeCount = (count: number) => {
        return count.toLocaleString() + '人';
    };

    return (
        <div className="overflow-x-auto">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>企業名</TableHead>
                        <TableHead>所在地</TableHead>
                        <TableHead>従業員数</TableHead>
                        <TableHead>契約タイプ</TableHead>
                        <TableHead>ステータス</TableHead>
                        <TableHead>契約開始日</TableHead>
                        <TableHead>契約終了日</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {companies.map((company) => (
                        <TableRow
                            key={company.id?.toString()}
                            className="group hover:bg-gray-50 transition-colors"
                        >
                            <TableCell className="relative p-0">
                                <Link
                                    href={`/dashboard/companies?companyId=${company.id}`}
                                    className="flex items-center w-full h-full p-4 text-gray-900 hover:text-blue-600 transition-colors font-semibold"
                                >
                                    <span>{company.company_name}</span>
                                    <MdChevronRight className="ml-2 h-6 w-6 text-gray-500 group-hover:text-blue-600" />
                                </Link>
                            </TableCell>
                            <TableCell>
                                <TruncatedText text={company.location} maxLength={20} />
                            </TableCell>
                            <TableCell>{formatEmployeeCount(company.employee_count)}</TableCell>
                            <TableCell>{contractTypeLabels[company.contract_type]}</TableCell>
                            <TableCell>
                                <StatusBadge status={company.status} />
                            </TableCell>
                            <TableCell>{new Date(company.affiliation_start_date).toLocaleDateString('ja-JP')}</TableCell>
                            <TableCell>{company.affiliation_end_date ? new Date(company.affiliation_end_date).toLocaleDateString('ja-JP') : "-"}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    );
};

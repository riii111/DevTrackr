import { WelcomeMessage } from "@/components/features/dashboard/WelcomeMessage";

const DashboardPage: React.FC = () => {
    return (
        <div className="text-text-primary">
            <WelcomeMessage />
            <h1 className="text-2xl font-bold mb-4">ダッシュボード</h1>
            <p>ここにダッシュボードの内容が表示されます。</p>
        </div>
    );
};

export default DashboardPage;
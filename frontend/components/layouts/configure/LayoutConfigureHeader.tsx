import AccountMenu from '@/components/molecules/core/AccountMenu';

interface LayoutConfigureHeaderProps {
  title: string;
}

const LayoutConfigureHeader: React.FC<LayoutConfigureHeaderProps> = ({ title }) => {
  return (
    <header className="bg-white shadow-sm w-full">
      <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center py-4">
        {/* <div className="flex items-center space-x-4">
          <h1 className="text-xl font-semibold text-gray-900">設定</h1>
          <span className="text-xl text-gray-700">{title}</span>
        </div> */}
        <AccountMenu />
      </nav>
    </header>
  );
};

export default LayoutConfigureHeader;
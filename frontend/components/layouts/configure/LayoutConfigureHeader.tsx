import AccountMenu from '@/components/molecules/core/AccountMenu';

interface LayoutConfigureHeaderProps {
  title: string;
}

const LayoutConfigureHeader: React.FC<LayoutConfigureHeaderProps> = ({ title }) => {
  return (
    <header className="bg-[#EBDFD7] w-full h-16 border-b border-gray-400">
      <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center h-full">
        <h1 className="text-xl font-semibold">{title}</h1>
        <div className="ml-auto">
          <AccountMenu />
        </div>
      </nav>
    </header>
  );
};

export default LayoutConfigureHeader;
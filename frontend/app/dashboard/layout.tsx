import LayoutDashboardNavigation from "@/components/layouts/dashboard/LayoutDashboardNavigation";
import LayoutDashboardHeader from "@/components/layouts/dashboard/LayoutDashboardHeader";

export default async function ConfigureLayout({
  children,
}: {
  children: React.ReactNode;
}) {

  return (
    <div className="flex h-screen bg-[#EBDFD7]">
      <LayoutDashboardNavigation />
      <div className="flex flex-col flex-grow overflow-hidden">
        <LayoutDashboardHeader />
        <main className="flex-row p-8 overflow-y-auto">
          <div className="bg-white bg-opacity-30 backdrop-filter backdrop-blur-sm p-6 rounded-lg">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
}
import LayoutDashboardNavigation from "@/components/layouts/dashboard/LayoutDashboardNavigation";
import LayoutDashboardHeader from "@/components/layouts/dashboard/LayoutDashboardHeader";
import { DrawerProvider } from "@/lib/store/useDrawerStore";

export default async function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {

  return (
    <DrawerProvider>
      <div className="bg-main-bg text-foreground h-screen flex">
        <LayoutDashboardNavigation />
        <div className="flex flex-col flex-grow overflow-hidden">
          <LayoutDashboardHeader />
          <main className="flex-row p-8 overflow-y-auto bg-main-bg">
            {children}
          </main>
        </div>
      </div>
    </DrawerProvider>
  );
}
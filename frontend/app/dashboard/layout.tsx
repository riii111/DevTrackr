import LayoutDashboardNavigation from "@/components/layouts/dashboard/LayoutDashboardNavigation";
import LayoutDashboardHeader from "@/components/layouts/dashboard/LayoutDashboardHeader";
import { DrawerProvider } from "@/lib/store/useDrawerStore";

export default async function DashboardLayout({
  children,
  modal
}: {
  children: React.ReactNode;
  modal: React.ReactNode;
}) {
  return (
    <DrawerProvider>
      <div className="bg-main-bg text-foreground h-screen flex">
        <LayoutDashboardNavigation />
        <div className="flex flex-col flex-grow min-w-0 overflow-hidden">
          <LayoutDashboardHeader />
          <main className="flex-grow p-8 overflow-auto bg-main-bg">
            {children}
            {modal}
          </main>
        </div>
      </div>
    </DrawerProvider>
  );
}

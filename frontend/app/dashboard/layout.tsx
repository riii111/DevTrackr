import LayoutDashboardNavigation from "@/components/layouts/LayoutDashboardNavigation";
import LayoutDashboardHeader from "@/components/layouts/LayoutDashboardHeader";
import { DrawerProvider } from "@/lib/store/useDrawerStore";
import { WorkLogProvider } from "@/lib/store/useWorkLogStore";
import { WorkLogDialog } from "@/components/features/work-logs/Dialog/WorkLogDialog";


export default async function DashboardLayout({
  children,
  modal
}: {
  children: React.ReactNode;
  modal: React.ReactNode;
}) {
  return (
    <WorkLogProvider>
      <DrawerProvider>
        <div className="bg-main-bg text-foreground h-screen flex">
          <LayoutDashboardNavigation />
          <div className="flex flex-col flex-grow min-w-0 overflow-hidden">
            <LayoutDashboardHeader />
            <main className="flex-grow p-8 overflow-auto bg-main-bg">
              <WorkLogDialog />
              {children}
              {modal}
            </main>
          </div>
        </div>
      </DrawerProvider>
    </WorkLogProvider>
  );
}

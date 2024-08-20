import LayoutConfigureNavigation from "@/components/layouts/configure/LayoutConfigureNavigation";
import LayoutConfigureHeader from "@/components/layouts/configure/LayoutConfigureHeader";
// import { fetchOrganizations } from "@/lib/api";

export default async function ConfigureLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  // const organizations = await fetchOrganizations();

  return (
    <div className="flex h-screen bg-[#EBDFD7]">
      {/* <LayoutConfigureNavigation organizations={organizations} /> */}
      <LayoutConfigureNavigation />
      <div className="flex flex-col flex-grow overflow-hidden">
        <LayoutConfigureHeader />
        <main className="flex-row p-8 overflow-y-auto">{children}</main>
      </div>
    </div>
  );
}
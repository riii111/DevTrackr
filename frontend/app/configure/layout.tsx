"use client";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useOrganizationStore } from "@/stores/organizationStore";
import { useContractsStore } from "@/stores/contractsStore";
import LayoutConfigureNavigation from "@/components/layouts/configure/LayoutConfigureNavigation";
import LayoutConfigureHeader from "@/components/layouts/configure/LayoutConfigureHeader";
import AtomsCoreSnackbar from "@/components/atoms/core/AtomsCoreSnackbar";

type Menu = {
  name: string;
  path: string;
  icon: string;
};

export default function ConfigureLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const router = useRouter();
  const organizationStore = useOrganizationStore();
  const contractsStore = useContractsStore();

  const [nav, setNav] = useState<{ menus: Menu[] } | null>(null);
  const [pageTitle, setPageTitle] = useState<string | undefined>();

  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        organizationStore.listOrganization(),
        contractsStore.fetchContract(),
      ]);
    };
    fetchData();
  }, []);

  useEffect(() => {
    if (nav?.menus) {
      const currentMenu = nav.menus.find((m) => router.asPath.includes(m.path));
      setPageTitle(currentMenu?.name);
    }
  }, [router.asPath, nav]);

  return (
    <div className="flex h-screen bg-backgroundGray">
      <LayoutConfigureNavigation
        ref={(el: { menus: Menu[] } | null) => setNav(el)}
      />
      <LayoutConfigureHeader title={pageTitle} />
      <main className="flex-row p-8 overflow-y-auto">{children}</main>
      {/* <AtomsCoreSnackbar /> */}
    </div>
  );
}

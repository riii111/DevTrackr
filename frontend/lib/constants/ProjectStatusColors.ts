import { ProjectStatus } from "@/types/project";

export const statusColors = {
  [ProjectStatus.Planning]:
    "bg-blue-100 text-blue-800 hover:bg-blue-100 hover:text-blue-800",
  [ProjectStatus.InProgress]:
    "bg-yellow-100 text-yellow-800 hover:bg-yellow-100 hover:text-yellow-800",
  [ProjectStatus.Completed]:
    "bg-green-100 text-green-800 hover:bg-green-100 hover:text-green-800",
  [ProjectStatus.OnHold]:
    "bg-gray-100 text-gray-800 hover:bg-gray-100 hover:text-gray-800",
  [ProjectStatus.Cancelled]:
    "bg-red-100 text-red-800 hover:bg-red-100 hover:text-red-800",
};

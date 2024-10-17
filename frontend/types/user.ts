export interface User {
  id: string;
  username: string;
  email: string;
  avatar: string;
  role: UserRole;
}

export const UserRole = {
  FrontEnd: "FrontEnd",
  BackEnd: "BackEnd",
  FullStack: "FullStack",
  DevOps: "DevOps",
  Security: "Security",
  ProductManager: "ProductManager",
  ProjectManager: "ProjectManager",
} as const;

export type UserRole = (typeof UserRole)[keyof typeof UserRole];

export interface UpdateUserRequest {
  username: string;
  email: string;
  password?: string;
  avatar?: string;
  role?: UserRole;
}

export interface UserResponse {
  user: User;
}

export interface AuthResponse {
  token_type: string;
  expires_in: number;
}

export interface AuthTokenCreatedResponse {
  message: string;
}

export interface User {
  id: string;
  username: string;
  email: string;
  avatar_url: string;
  role: UserRole;
  created_at: string;
  updated_at: string;
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
  avatar?: string | null; // null = アバターを削除する場合
  role?: UserRole;
}

export type UserResponse = User;

export interface AuthResponse {
  token_type: string;
  expires_in: number;
}

export interface AuthTokenCreatedResponse {
  message: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

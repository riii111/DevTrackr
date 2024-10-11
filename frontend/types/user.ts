export interface User {
  id: string;
  username: string;
  email: string;
  icon: string;
  role: UserRole;
}

export enum UserRole {
  Frontend = "Front-end",
  Backend = "Back-end",
  Fullstack = "Full-stack",
  Designer = "Designer",
  Infrastructure = "Infrastructure",
  ProductManager = "Product Manager",
  ProjectManager = "Project Manager",
}

export interface UpdateUserRequest {
  username?: string;
  email?: string;
  icon?: string;
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

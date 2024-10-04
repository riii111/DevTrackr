export interface User {
  _id: string;
  username: string;
  email: string;
}

export interface AuthResponse {
  token_type: string;
  expires_in: number;
  access_token: string;
  refresh_token: string;
}

export interface AuthTokenCreatedResponse {
  message: string;
}

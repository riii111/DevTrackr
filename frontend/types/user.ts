export interface User {
  _id: string;
  username: string;
  email: string;
}

export interface AuthResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
}

export interface AuthTokenCreatedResponse {
  message: string;
}

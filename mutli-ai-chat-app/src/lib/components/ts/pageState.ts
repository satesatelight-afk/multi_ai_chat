export interface PageState {
  textarea_question: string;
  diplay_question: string;
  answer: string;
  chat_history: ChatHistory[];
  is_loading: boolean;
}

export interface ChatHistory {
  role: Role;
  chat: string;
}

export enum Role {
  System,
  User,
  ChatGPT,
}

export function role_to_string(role: Role): string {
  if (role == Role.System) {
    return "System";
  } else if (role == Role.User) {
    return "User";
  } else {
    return "ChatGPT";
  }
}

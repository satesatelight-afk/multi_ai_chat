import { invoke } from "@tauri-apps/api/core";
import type { PageState } from "./pageState";

export async function writeChatHistoryJson(
  json_name: string,
  pageState: PageState
): Promise<void> {
  await invoke("write_chat_history_Json", {
    json_name: json_name,
    chat_history: pageState.chat_history,
  });
}

export async function loadChatHistoryJson(
  json_name: string,
  pageState: PageState
): Promise<void> {
  pageState.chat_history = await invoke("load_chat_history_Json", {
    json_name: json_name,
    chat_history: pageState.chat_history,
  });
}

export async function loadChatHistoryTitles(): Promise<string[]> {
  let chat_history_titles: string[] = await invoke(
    "load_chat_history_titles",
    {}
  );
  return chat_history_titles;
}

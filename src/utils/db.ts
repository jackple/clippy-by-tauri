import { invoke } from "@tauri-apps/api/core"

export enum RecordType {
  Text = "text",
  Image = "image",
  File = "file",
}

export interface Record {
  id: number
  record_type: RecordType
  value: string
  thumbnail?: string
  size?: number
  img_size?: string
  created_at: string
  updated_at: string
}

export async function addRecord(
  params: Omit<Record, "id" | "created_at" | "updated_at">
): Promise<number> {
  console.log("addRecord", params)
  return invoke("add_record", { record: params })
}

export async function getRecords(): Promise<Record[]> {
  return invoke("get_records")
}

export async function deleteRecord(id: number): Promise<void> {
  return invoke("delete_record", { id })
}

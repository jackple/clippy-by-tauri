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

interface QueryParams {
  last_updated_at?: string
  limit: number
  keyword?: string
}

export async function getRecords(params: QueryParams): Promise<Record[]> {
  return invoke("get_records", { params })
}

export async function deleteRecord(id: number): Promise<void> {
  return invoke("delete_record", { id })
}

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
  is_deleted?: boolean
  favorite: boolean
}

interface QueryParams {
  last_updated_at?: string
  limit: number
  keyword?: string
  record_type?: RecordType
  favorite?: boolean
}

export async function getRecords(params: QueryParams): Promise<Record[]> {
  return invoke("get_records", { params })
}

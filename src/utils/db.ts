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
  /** 前端自定义属性, 解决文本过长时卡顿 */
  display_text?: string
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
  let data: Record[] = await invoke("get_records", { params })
  data = data.map((r) => {
    if (r.record_type === "text" && r.value.length > 250) {
      return { ...r, display_text: r.value.slice(0, 250) }
    }
    return r
  })
  return data
}

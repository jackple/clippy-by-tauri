import { useCallback, useState } from "react"
import classNames from "classnames"

import { RecordType } from "../../utils/db"
import styles from "./styles.module.scss"

interface Props {
  onSearch: (keyword: string, type: RecordType | "all") => void
  onBlur?: () => void
}

const TYPES = [
  { label: "全部", value: "all" },
  { label: "文本", value: RecordType.Text },
  { label: "图片", value: RecordType.Image },
  { label: "文件", value: RecordType.File },
  { label: "收藏", value: "favorite" },
]

export function Search({ onSearch, onBlur }: Props) {
  const [value, setValue] = useState("")
  const [selectedType, setSelectedType] = useState<RecordType | "all">("all")

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      // 阻止方向键事件冒泡，避免触发列表的左右滚动
      if (["ArrowLeft", "ArrowRight"].includes(e.key)) {
        return e.stopPropagation()
      }

      if (e.key === "Enter") {
        e.stopPropagation()
        e.currentTarget.blur()
        onBlur?.()
      }
    },
    [onBlur]
  )

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = e.target.value
      setValue(newValue)
      onSearch(newValue, selectedType)
    },
    [onSearch, selectedType]
  )

  const handleClear = useCallback(() => {
    setValue("")
    onSearch("", selectedType)
  }, [onSearch, selectedType])

  const handleTypeChange = useCallback(
    (type: RecordType | "all" | "favorite") => {
      setSelectedType(type as RecordType | "all")
      // 当切换到图片类型时，清空搜索框
      if (type === RecordType.Image) {
        setValue("")
        onSearch("", type as RecordType | "all")
      } else {
        onSearch(value, type as RecordType | "all")
      }
    },
    [onSearch, value]
  )

  return (
    <div className={styles.container}>
      <div className={styles.searchWrapper}>
        <i className={styles.searchIcon} />
        <input
          type="text"
          value={value}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          placeholder="搜索..."
          className={classNames(styles.searchInput, {
            [styles.disabled]: selectedType === RecordType.Image,
          })}
          disabled={selectedType === RecordType.Image}
        />
        {value && (
          <button
            type="button"
            className={styles.clearButton}
            onClick={handleClear}
            aria-label="清除搜索"
          />
        )}
      </div>
      <div className={styles.types}>
        {TYPES.map((type) => (
          <button
            key={type.value}
            className={classNames(styles.typeButton, {
              [styles.active]: selectedType === type.value,
            })}
            onClick={() => handleTypeChange(type.value as RecordType | "all")}
          >
            {type.label}
          </button>
        ))}
      </div>
    </div>
  )
}

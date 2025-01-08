import { useCallback, useState } from "react"
import styles from "./styles.module.scss"

interface Props {
  onSearch: (keyword: string) => void
}

export function SearchInput({ onSearch }: Props) {
  const [value, setValue] = useState("")

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === "Enter") {
        onSearch(value)
      }
      // 阻止方向键事件冒泡，避免触发列表的左右滚动
      if (["ArrowLeft", "ArrowRight"].includes(e.key)) {
        e.stopPropagation()
      }
    },
    [value, onSearch]
  )

  const handleClear = useCallback(() => {
    setValue("")
    onSearch("")
  }, [onSearch])

  return (
    <div className={styles.container}>
      <div className={styles.searchWrapper}>
        <i className={styles.searchIcon} />
        <input
          type="text"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="搜索..."
          className={styles.searchInput}
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
    </div>
  )
}

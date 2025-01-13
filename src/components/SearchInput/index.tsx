import { useCallback, useState } from "react"
import styles from "./styles.module.scss"

interface Props {
  onSearch: (keyword: string) => void
}

export function SearchInput({ onSearch }: Props) {
  const [value, setValue] = useState("")

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      // 阻止方向键事件冒泡，避免触发列表的左右滚动
      if (["ArrowLeft", "ArrowRight"].includes(e.key)) {
        e.stopPropagation()
      }
    },
    []
  )

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = e.target.value
      setValue(newValue)
      onSearch(newValue)
    },
    [onSearch]
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
          onChange={handleChange}
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

import { useState, useEffect, useCallback } from "react";

interface Position {
  x: number;
  y: number;
}

// ドラッグ可能な要素を制御するカスタムフック
export const useDraggable = (
  initialPosition: Position,
  dialogRef: React.RefObject<HTMLElement>
) => {
  // 現在の位置
  const [position, setPosition] = useState(initialPosition);
  // ドラッグ中かどうか
  const [isDragging, setIsDragging] = useState(false);
  // ドラッグ開始位置
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });

  // マウスダウン時の処理
  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      if (e.target instanceof Element && e.target.closest(".drag-handle")) {
        setIsDragging(true);
        setDragStart({
          x: e.clientX - position.x,
          y: e.clientY - position.y,
        });
      }
    },
    [position]
  );

  // ドラッグ中の処理
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (isDragging) {
        const newX = e.clientX - dragStart.x;
        const newY = e.clientY - dragStart.y;

        // 要素がウィンドウ外に出ないように制限
        const maxX = window.innerWidth - (dialogRef.current?.offsetWidth || 0);
        const maxY =
          window.innerHeight - (dialogRef.current?.offsetHeight || 0);

        setPosition({
          x: Math.min(Math.max(0, newX), maxX),
          y: Math.min(Math.max(0, newY), maxY),
        });
      }
    };

    // マウスアップ時の処理
    const handleMouseUp = () => {
      setIsDragging(false);
    };

    // ドラッグ中のみイベントリスナーを追加
    if (isDragging) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    }

    // クリーンアップ関数
    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging, dragStart, dialogRef]);

  return { position, isDragging, handleMouseDown };
};

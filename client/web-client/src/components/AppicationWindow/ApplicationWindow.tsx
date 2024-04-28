import { ReactElement, useCallback, useEffect, useRef, useState } from "react";
import { WindowTitleBar } from "../TitleBar/WindowTitleBar";
import "./ApplicationWindow.css";

type Props = {
  defaultWindowName: string;
  children: ReactElement;
};

type WindowPosition = {
  x: number;
  y: number;
};

export const ApplicationWindow = ({ defaultWindowName, children }: Props) => {
  const [position, setPosition] = useState({ x: 10, y: 10 } as WindowPosition);
  const [dragging, setDragging] = useState(false);
  const [windowName, setWindowName] = useState(defaultWindowName);

  const draggingOffset = useRef({ x: 0, y: 0 } as WindowPosition);

  const onMouseMove = useCallback(
    (event: MouseEvent) => {
      if (dragging) {
        const x = event.clientX - draggingOffset.current.x;
        const y = event.clientY - draggingOffset.current.y;
        setPosition({ x, y } as WindowPosition);
      }
    },
    [dragging]
  );

  const onMouseDown = useCallback(
    (event: React.MouseEvent) => {
      draggingOffset.current.x = event.clientX - position.x;
      draggingOffset.current.y = event.clientY - position.y;
      setDragging(true);
    },
    [position.x, position.y]
  );

  const onMouseUp = useCallback(() => {
    setDragging(false);
  }, [dragging]);

  useEffect(() => {
    if (dragging) {
      const handler = (event: MouseEvent) => onMouseMove(event);
      window.addEventListener("mousemove", handler);
      return () => window.removeEventListener("mousemove", handler);
    }
  }, [dragging, onMouseMove]);

  useEffect(() => {
    if (dragging) {
      const handler = () => onMouseUp();
      window.addEventListener("mouseup", handler);
      return () => window.removeEventListener("mouseup", handler);
    }
  }, [dragging, onMouseUp]);

  return (
    <div
      className="application-window"
      style={{ top: position.y, left: position.x }}
    >
      <WindowTitleBar
        onCloseClicked={() => {}}
        onMouseDown={onMouseDown}
        title={windowName}
      ></WindowTitleBar>
      <div className="application-window--content">{children}</div>
    </div>
  );
};

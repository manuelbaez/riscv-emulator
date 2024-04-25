import { CSSProperties, ReactNode } from "react";
import "./RoundButton.css";

type Props = {
  onClick: () => void;
  children: ReactNode;
  type?: "error" | "warning" | "success";
  style?: CSSProperties;
};

export const RoundButton = ({
  type = "success",
  onClick,
  children,
  ...other
}: Props) => {
  return (
    <div
      className={`round-button round-button--${type}`}
      {...other}
      onClick={onClick}
    >
      <span className="round-button--text">{children}</span>
    </div>
  );
};

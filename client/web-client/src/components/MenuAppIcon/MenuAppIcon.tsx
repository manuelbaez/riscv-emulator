import { CSSProperties } from "react";
import "./MenuAppIcon.css";
type Props = {
  iconUrl: string;
  name: string;
  displayName?: boolean;
  style?: CSSProperties;
  iconStyle?: CSSProperties;
  iconScale?: number;
  className?: string;
};
export const MenuAppIcon = ({
  iconUrl,
  name,
  className,
  style,
  iconStyle,
  iconScale = 1,
  displayName = true,
}: Props) => {
  return (
    <div className={`menu-app-icon-container ${className}`} style={style}>
      <img
        className="menu-app-icon--icon"
        src={iconUrl}
        style={{ zoom: iconScale, ...iconStyle }}
      ></img>
      {displayName && <p>{name}</p>}
    </div>
  );
};

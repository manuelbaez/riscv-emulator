import { MenuAppIcon } from "../MenuAppIcon/MenuAppIcon";
import "./DockBar.css";

export const DockBar = () => {
  return (
    <div className="dock-bar">
      <MenuAppIcon
        name="Terminal"
        displayName={false}
        className="bar-icon"
        iconScale={1.35}
        iconUrl="public/default-apps-icons/terminal.webp"
      />
      <MenuAppIcon
        name="Apps"
        displayName={false}
        className="bar-icon"
        iconUrl="public/default-apps-icons/apps-menu.png"
        iconScale={0.95}
        iconStyle={{ color: "white", filter: "invert(100%)" }}
      />
    </div>
  );
};

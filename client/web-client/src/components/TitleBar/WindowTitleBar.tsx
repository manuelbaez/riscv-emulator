import { RoundButton } from "../RoundButton/RoundButton";
import CloseIcon from "@mui/icons-material/Close";
import MinimizeIcon from "@mui/icons-material/Minimize";
import MaximizeIcon from "@mui/icons-material/Maximize";
import { MouseEvent } from "react";

import "./WindowTitleBar.css";
type Props = {
  title: string;
  onCloseClicked: () => void;
  onMaximizeClicked?: () => void;
  onMinimizeClicked?: () => void;
  onMouseDown?: (event: MouseEvent) => void;
  onMouseUp?: (event: MouseEvent) => void;
  onMouseMove?: (event: MouseEvent) => void;
};

export const WindowTitleBar = ({
  title,
  onCloseClicked,
  onMaximizeClicked = () => {},
  onMinimizeClicked = () => {},
  onMouseDown = () => {},
  onMouseUp = () => {},
  onMouseMove = () => {},
}: Props) => {
  return (
    <div
      className="window-title-bar"
      onMouseDown={onMouseDown}
      onMouseUp={onMouseUp}
      onMouseMove={onMouseMove}
    >
      <div>
        <span>{title}</span>
      </div>
      <div className="window-title-bar--button-row">
        <RoundButton
          type="success"
          style={{ marginRight: "5px", color: "black" }}
          onClick={onMinimizeClicked}
        >
          <MinimizeIcon style={{ fontSize: "15px" }} />
        </RoundButton>
        <RoundButton
          type="warning"
          style={{ marginRight: "5px", color: "black" }}
          onClick={onMaximizeClicked}
        >
          <MaximizeIcon style={{ marginTop: "3px", fontSize: "15px" }} />
        </RoundButton>
        <RoundButton
          type="error"
          style={{ color: "black" }}
          onClick={onCloseClicked}
        >
          <CloseIcon style={{ fontSize: "15px" }} />
        </RoundButton>
      </div>
    </div>
  );
};

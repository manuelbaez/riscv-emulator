import { useCallback, useEffect, useState } from "react";
import "./TopBar.css";

export const TopBar = () => {
  const [date, setDate] = useState(new Date());

  useEffect(() => {
    setInterval(() => {
      setDate(new Date());
    }, 1000);
  }, []);

  return (
    <div className="top-bar">
      <div></div>
      <div className="bar--clock">{`${date.toDateString()} - ${date.toLocaleTimeString()}`}</div>
      <div></div>
    </div>
  );
};

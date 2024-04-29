import { ApplicationWindow } from "./components/Window/AppicationWindow/ApplicationWindow";
import "./App.css";
import { DockBar } from "./components/DockBar/DockBar";
import { TopBar } from "./components/TopBar/TopBar";
function App() {
  return (
    <>
      <TopBar></TopBar>
      <ApplicationWindow defaultWindowName="Test 2">
        <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/SEG_DVD_430_-_Printed_circuit_board-4276.jpg/1200px-SEG_DVD_430_-_Printed_circuit_board-4276.jpg"></img>
      </ApplicationWindow>
      <ApplicationWindow defaultWindowName="Test 1">
        <img src="https://i.ytimg.com/vi/x4hYUVgU5fs/maxresdefault.jpg"></img>
      </ApplicationWindow>
      <DockBar></DockBar>
    </>
  );
}

export default App;

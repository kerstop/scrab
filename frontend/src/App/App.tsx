import * as React from "react";
import "./App.scss";
import ViewWindow from "../ViewWindow/ViewWindow";
import Controls from "../Controls/Controls";
import Room from "../Room/Room";

function App() {
  return (
    <div className="App">
      <ViewWindow>
        <Room name="[0,0,0]" />
      </ViewWindow>
      <Controls />
    </div>
  );
}

export default App;

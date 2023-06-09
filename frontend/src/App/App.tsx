import * as React from "react";
import "./App.scss";
import ViewWindow from "../ViewWindow/ViewWindow";
import Controls from "../Controls/Controls";
import Room from "../Room/Room";
import { Cordinate } from "../__generated__/graphql";

function App() {

  let [room, setRoom] = React.useState<Cordinate>({q:0,r:0,s:0})

  return (
    <div className="App">
      <ViewWindow>
        <Room currentRoom={room}/>
      </ViewWindow>
      <Controls changeRoom={setRoom} currentRoom={room}/>
    </div>
  );
}

export default App;

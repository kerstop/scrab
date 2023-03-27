import * as React from "react";
import "./App.scss";
import ViewWindow from "../ViewWindow/ViewWindow";
import Controls from "../Controls/Controls";
import Room from "../Room/Room";
import * as scrab from "../scrab_frontend_types";

function App() {
  let [world, setWorld] = React.useState<scrab.PublicWorld>();

  React.useEffect(() => {
    fetch("http://localhost:8080/world/")
      .then((data) => {
        return data.json();
      })
      .then((json) => {
        setWorld(json);
      });
  }, []);

  return (
    <div className="App">
      <ViewWindow>
        {world !== undefined ? (
          world.rooms.filter((_, i)=> i < 37).map((room, i) => {
            return <Room
              name={room.name}
              x={room.screen_space_x}
              y={room.screen_space_y}
            />
        })
        ) : (
          <Room name="[0,0,0]" />
        )}
      </ViewWindow>
      <Controls />
    </div>
  );
}

export default App;

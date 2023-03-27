import Tile from "../Tile/Tile";
import * as React from "react";
import * as scrab from "../scrab_frontend_types";

interface RoomArgs {
  name: string;
  x?: number;
  y?: number;
}

export default function Room(args: RoomArgs) {
  let [room, setRoom] = React.useState<scrab.PublicRoom>();

  React.useEffect(() => {
    fetch(`http://localhost:8080/world/${args.name}`)
      .then((resp) => {
        return resp.json();
      })
      .then((data) => {
        setRoom(data);
      });
  }, []);

  if (room !== undefined) {
    return (
      <g transform={`translate(${args.x??0}, ${args.y??0})`}>
        {room.tiles.map((tile, i: any) => {
          return (
            <Tile
              key={i}
              wall={tile.wall}
              x={tile.x}
              y={tile.y}
            />
          );
        })}
      </g>
    );
  } else {
    return <p>"Loading"</p>;
  }
}

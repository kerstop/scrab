import Tile from "../Tile/Tile";
import * as React from "react";
import * as scrab from "../scrab_frontend_types";
import * as Hex from "../hex_utils";

interface RoomArgs {
  name: string;
  x?: number;
  y?: number;
}

const tile_spacing = 100;

export default function Room(args: RoomArgs) {
  let [room, setRoom] = React.useState<scrab.PubRoom>();

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
          let cord = new Hex.Cordinate(tile.cord.q,tile.cord.r,tile.cord.s);
          let [x,y] = cord.toPixelFlat();
          return (
            <Tile
              key={i}
              wall={tile.wall}
              x={x * tile_spacing}
              y={y * tile_spacing}
            />
          );
        })}
      </g>
    );
  } else {
    return <p>"Loading"</p>;
  }
}

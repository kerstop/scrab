import Tile from "../Tile/Tile";
import * as React from "react";
import { useQuery } from "@apollo/client";
import * as Hex from "../hex_utils";
import { gql } from "../__generated__";

const GET_ROOM_INFO = gql(`
  query GetRoomInfo {
    world {
      room(q: 0, r: 0, s: 0) {
        cordinate {
          q
          r
          s
        }

        tiles {
          isWall
          cordinate {
            q
            r
            s
          }
        }
      }
    }
  }
`);

interface RoomArgs {
  name: string;
  x?: number;
  y?: number;
}

const TILE_SPACING = 100;

export default function Room(args: RoomArgs) {
  const { loading, error, data } = useQuery(GET_ROOM_INFO);

  if (loading) console.log("loading...");
  if (error) console.log(error);
  if (data) console.log(data);

  if (data) {
    const {
      world: { room },
    } = data;

    return (
      <g transform={`translate(${args.x ?? 0}, ${args.y ?? 0})`}>
        {room.tiles.map((tile, i: any) => {
          let [x, y] = Hex.toPixelFlat(tile.cordinate);
          return (
            <Tile
              key={i}
              wall={tile.isWall}
              x={x * TILE_SPACING}
              y={y * TILE_SPACING}
            />
          );
        })}
      </g>
    );
  } else if (loading) {
    return <p>"Loading"</p>;
  } else {
    return <p>{error?.message}</p>;
  }
}

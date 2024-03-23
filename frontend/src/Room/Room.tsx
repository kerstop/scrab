import Tile from "../Tile/Tile";
import * as React from "react";
import { useQuery } from "@apollo/client";
import * as Hex from "../hex_utils";
import { Cordinate } from "../hex_utils";
import { gql } from "../__generated__";
import "./Room.scss";

const GET_ROOM_INFO = gql(`
  query GetRoomInfo($q: Int = 0,$r: Int = 0,$s: Int = 0) {
    world {
      room(q: $q, r: $r, s: $s) {
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
  x?: number;
  y?: number;
  currentRoom: Cordinate; 
}

const TILE_SPACING = 100;

export default function Room(args: RoomArgs) {
  const { currentRoom: {q,r,s} } = args;

  const { loading, error, data } = useQuery(GET_ROOM_INFO, {
    variables: { q, r, s },
  });

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
    return <text className="error-text">Loading</text>;
  } else {
    return <text className="error-text">{error?.message}</text>;
  }
}

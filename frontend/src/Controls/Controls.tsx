import { useState } from "react";
import * as React from "react";
import { UNIT_CORDINATES, toPixelFlat, toPixelPoint } from "../hex_utils";
import "./Controls.scss";
import { Cordinate } from "../__generated__/graphql";

interface ControlsArgs {
  changeRoom: (new_room: Cordinate) => void;
  currentRoom: Cordinate;
}

export default function Controls(args: ControlsArgs) {
  let { q, r, s } = args.currentRoom;
  return (
    <div className="controls">
      <p>{`Current room: [${q},${r},${s}]`}</p>
      {Compass(args)}
    </div>
  );
}

function Compass(args: ControlsArgs) {
  let { q, r, s } = args.currentRoom;
  return (
    <svg className="compass" viewBox="-300 -300 600 600">
      <polygon
        className="current-room"
        points="100,0 50,-87 -50,-87 -100,-0 -50,87 50,87"
      />
      {Object.entries(UNIT_CORDINATES).map(([k, v]) => {
        let [x, y] = toPixelPoint(v);
        return (
          <polygon
            key={k}
            className="other-rooms"
            points="100,0 50,-87 -50,-87 -100,-0 -50,87 50,87"
            style={
              {
                "--x": `${x * 90}px`,
                "--y": `${y * 90}px`,
              } as React.CSSProperties
            }
            onClick={() => {
              args.changeRoom({
                q: q + v.q,
                r: r + v.r,
                s: s + v.s,
              });
            }}
          />
        );
      })}
      <text
        style={
          {
            "--x": `${Math.sin(2*Math.PI/3) * 250}px`,
            "--y": `${Math.cos(2*Math.PI/3) * 250}px`,
          } as React.CSSProperties
        }
      >
        q+
      </text>
      <text
        style={
          {
            "--x": `${Math.sin(0) * 250}px`,
            "--y": `${Math.cos(0) * 250}px`,
          } as React.CSSProperties
        }
      >
        r+
      </text>
      <text
        style={
          {
            "--x": `${Math.sin(4*Math.PI/3) * 250}px`,
            "--y": `${Math.cos(4*Math.PI/3) * 250}px`,
          } as React.CSSProperties
        }
      >
        s+
      </text>
    </svg>
  );
}

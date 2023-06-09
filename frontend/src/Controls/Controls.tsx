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
  let {q,r,s} = args.currentRoom;
  return (
    <div className="controls">
      <p>{`Current room: [${q},${r},${s}]`}</p>
      <svg className="compass">
        <g>
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
                    "--x": `${x * 100}px`,
                    "--y": `${y * 100}px`,
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
        </g>
      </svg>
    </div>
  );
}

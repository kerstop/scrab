import "./ViewWindow.scss";
import * as React from "react";
import Tile from "../Tile/Tile";
import * as scrab from "../scrab_frontend_types";

function ViewWindow() {
  let [room, setRoom] = React.useState<scrab.PublicRoom>();
  let [x, setX] = React.useState(0);
  let [y, setY] = React.useState(0);
  let [zoom, setZoom] = React.useState(1);
  let delta = React.useRef<[number, number]>(null);
  let [listenersMounted, setListenersMounted] = React.useState(false);
  let window = React.useRef<SVGSVGElement>(null);

  function get_room() {
    return fetch("http://localhost:8080/world/[0,0,0]").then((resp) => {
      return resp.json();
    });
  }

  React.useEffect(() => {
    get_room().then((data) => {
      setRoom(data);
    });
  }, []);

  React.useEffect(() => {
    if (window.current !== null && !listenersMounted) {
      window.current.addEventListener(
        "wheel",
        (e) => {
          e.preventDefault();
          let direction = e.deltaY > 0 ? 1.1 : 0.9;
          setZoom((zoom *= direction));
        },
        { passive: false }
      );
      setListenersMounted(true);
    }
  });

  if (room === undefined) {
    return <svg className="ViewWindow"></svg>;
  } else {
    return (
      <svg
        ref={window}
        className="ViewWindow"
        onMouseDown={(e) => {
          if (window.current !== null) {
            window.current.onmousemove = (e) => {
              console.log(`(${e.movementX}, ${e.movementY})`);
              setX((x += e.movementX / zoom));
              setY((y += e.movementY / zoom));
            };
          }
        }}
        onMouseUp={() => {
          if (window.current !== null) {
            window.current.onmousemove = null;
          }
        }}
        onMouseLeave={() => {
          if (window.current !== null) {
            window.current.onmousemove = null;
          }
        }}
      >
        <g
          transform={`translate(${350 + x * zoom},${
            350 + y * zoom
          }) scale(${zoom})`}
        >
          {room.tiles.map((tile, i: any) => {
            return <Tile key={i} wall={tile.wall} x={tile.x} y={tile.y} />;
          })}
        </g>
      </svg>
    );
  }
}

export default ViewWindow;

import "./ViewWindow.scss";
import * as React from "react";
import Tile from "../Tile/Tile";
import Room from "../Room/Room";

interface ViewWindowInterface {
  children?: JSX.Element | JSX.Element[];
}

function ViewWindow(args: ViewWindowInterface) {
  let [x, setX] = React.useState(0);
  let [y, setY] = React.useState(0);
  let [viewSize, setViewSize] = React.useState([0, 0]);
  let [zoom, setZoom] = React.useState(1);
  let [listenersMounted, setListenersMounted] = React.useState(false);
  let window = React.useRef<SVGSVGElement>(null);

  React.useEffect(() => {
    if (window.current !== null && !listenersMounted) {
      window.current.addEventListener(
        "wheel",
        (e) => {
          e.preventDefault();
          let direction = e.deltaY > 0 ? 0.8 : 1.2;
          setZoom((zoom *= direction));
        },
        { passive: false }
      );
      setListenersMounted(true);
      setViewSize([window.current.clientWidth, window.current.clientHeight]);
    }
  });

  return (
    <svg
      ref={window}
      className="ViewWindow"
      onMouseDown={(e) => {
        if (window.current !== null) {
          window.current.onmousemove = (e) => {
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
        transform={`translate(${viewSize[0] / 2 + x * zoom},${
          viewSize[1] / 2 + y * zoom
        }) scale(${zoom})`}
      >
        {args.children}
      </g>
    </svg>
  );
}

export default ViewWindow;

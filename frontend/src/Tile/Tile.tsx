import * as React from "react";
import "./Tile.css"

interface TileProps {
    x: number,
    y: number,
    wall: boolean,
}

function Tile(props: TileProps) {
    let color = props.wall ? "#111111" : "#444444"

    return <polygon
    fill={color}
    transform={`translate(${props.x}, ${props.y}) scale(0.8)`}
    points="100,0 50,-87 -50,-87 -100,-0 -50,87 50,87"
    />;
}

export default Tile;
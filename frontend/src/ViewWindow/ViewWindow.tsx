import "./ViewWindow.css";
import * as React from "react";
import Tile from "../Tile/Tile";

function ViewWindow() {
    let cords: [number, number][] = [
        [0, 0],
        [0, -34],
        [30, -17],
        [30, 17],
        [0, 34],
        [-30, 17],
        [-30, -17],
        [0, -69],
        [30, -51],
        [60, -34],
        [60, 0],
        [60, 34],
        [30, 51],
        [0, 69],
        [-30, 51],
        [-60, 34],
        [-60, 0],
        [-60, -34],
        [-30, -51],
        [0, -103],
        [30, -86],
        [60, -69],
        [90, -51],
        [90, -17],
        [90, 17],
        [90, 51],
        [60, 69],
        [30, 86],
        [0, 103],
        [-30, 86],
        [-60, 69],
        [-90, 51],
        [-90, 17],
        [-90, -17],
        [-90, -51],
        [-60, -69],
        [-30, -86],
    ];

    return <div className="ViewWindow">
        {cords.map((cord, i) => {
            return <Tile key={i} x={cord[0] + 250} y={cord[1] + 250}></Tile>
        })}
    </div>
}

export default ViewWindow;
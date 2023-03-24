import "./ViewWindow.css";
import * as React from "react";
import Tile from "../Tile/Tile";
import * as scrab from "../scrab_frontend_types";

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

    let [room, setRoom] = React.useState<scrab.PublicRoom>();

    function get_room() {
        return fetch("http://localhost:8080/world/[0,0,0]")
            .then((resp) => {
                return resp.json()
            })
    }

    React.useEffect(() => {
        get_room().then((data) => {
            setRoom(data);
        })
    }, [])

    console.log(room);

    if (room === undefined) {
        return (
            <svg className="ViewWindow">
                <g transform="translate(250,250) scale(1.5)">
                    {cords.map((cord, i) => {
                        return <Tile key={i} x={cord[0]} y={cord[1]} />
                    })}
                </g>
            </svg>
        )
    } else {
        return (
<svg className="ViewWindow">
                <g transform="translate(350,350) scale(0.1)">
                    {room.tiles.map((tile, i:any) => {
                        return <Tile key={i} x={tile.x} y={tile.y} />
                    })}
                </g>
            </svg>
        )
    }
}

export default ViewWindow;
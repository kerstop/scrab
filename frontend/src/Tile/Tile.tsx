import * as React from "react";
import "./Tile.css"

interface TileProps {
    x: number,
    y: number,
}

function Tile(props: TileProps) {

    let tile: React.MutableRefObject<HTMLDivElement | null> = React.useRef(null);

    React.useEffect(() => {
        if (tile.current != null) {
            tile.current.style.setProperty("--x-pos", props.x.toString()+"px");
            tile.current.style.setProperty("--y-pos", props.y.toString()+"px");
        }
    }, [])

    return <div ref={tile} className="Tile"></div>;
}

export default Tile;
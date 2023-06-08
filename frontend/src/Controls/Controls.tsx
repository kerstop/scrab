import { useState } from "react";
import "./Controls.scss";

export default function Controls() {
  return (
    <div className="controls">
      <p>hello</p>
      <svg className="compass">
        <polygon
          className="current-room"
          points="100,0 50,-87 -50,-87 -100,-0 -50,87 50,87"
        />
      </svg>
    </div>
  );
}

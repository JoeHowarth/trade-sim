import Konva, { Stage, Layer } from "react-konva";
import React, { useCallback, useEffect, useRef, useState } from "react";

export default (props) => {
  const { children } = props;
  return (
    <Stage
      style={{ position: "absolute", top: 0, right: 0, zIndex: 1 }}
      width={window.innerWidth}
      height={window.innerHeight}
      {...props}
    >
      <Layer>{children}</Layer>
    </Stage>
  );
};
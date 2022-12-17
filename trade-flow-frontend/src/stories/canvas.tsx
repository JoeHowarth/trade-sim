import Konva, { Stage, Layer, KonvaNodeComponent } from "react-konva";
import React, {
  ReactElement,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { NodeConfig, Node } from "konva/types/Node";

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

// Note: all components in OverlayDom must have pointerEvents: "auto" set 
export const CanvasWithOverlay = (props: {
  OverlayDom: React.ReactNode[];
  children: React.ReactNode[];
  border?: boolean;
  width?: number;
  height?: number;
  domStyle?: Record<string, number | string>
}) => {
  const { children, OverlayDom } = props;

  // console.log(OverlayDom)
  const containerRef = useRef(null);
  const [{ width, height }, setWidthHeight] = useState({ width: 0, height: 0 });
  useEffect(() => {
    setWidthHeight({
      width: containerRef.current.offsetWidth,
      height: containerRef.current.offsetHeight,
    });
  }, []);
  return (
    <div
      ref={containerRef}
      id={"stage-container"}
      style={{
        // position: "absolute",
        // top: 0,
        // left: 0,
          // width: "100%",
          // height: "100%",
        width: width ? width : window.innerWidth,
        height: height ? height : window.innerHeight,
      }}
    >
      <Stage
        style={{ position: "relative", top: 0, right: 0, zIndex: 1 }}
        width={width}
        height={height}
      >
        <Layer>{children}</Layer>
      </Stage>
      <div
        style={{
          pointerEvents: "none",
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          zIndex: 3,
          // width: "fit-content",
          border: props.border ? "1px solid black" : "",
          ...props.domStyle
        }}
      >
        {OverlayDom}
      </div>
    </div>
  );
};

export function dbg<T>(x: T): T {
  console.log(x)
  return x
}
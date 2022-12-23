import React from "react";
import { PropsWithChildren, useEffect, useRef, useState } from "react";
import { transform } from "../../utils"

export interface PosProps {
  position: Point;
}

export const View = (
  props: PropsWithChildren<PosProps & { height?: number }>
) => {
  return (
    <CenteredAbove
      position={transform(props.position, {
        x: 0,
        y: props.height ? -props.height : -30,
      })}
    >
      {props.children}
    </CenteredAbove>
  );
};

export const AtPosition = (
  props: PropsWithChildren<
    PosProps & { offset?(ref: React.MutableRefObject<any>): Point }
  >
) => {
  const ref = useRef(null);
  const [offset, setOffset] = useState({ x: -10000, y: -10000 });
  useEffect(() => {
    if (ref.current) {
      setOffset(props.offset ? props.offset(ref) : { x: 0, y: 0 });
    }
  }, [ref.current]);

  return (
    <div
      style={{
        position: "absolute",
        top: props.position.y - offset.y,
        left: props.position.x - offset.x,
      }}
      ref={ref}
    >
      {props.children}
    </div>
  );
};

export const Centered = (props: PropsWithChildren<PosProps>) => {
  return (
    <AtPosition
      offset={(ref) => ({
        x: ref.current.offsetWidth / 2,
        y: ref.current.offsetHeight / 2,
      })}
      {...props}
    />
  );
};

export const CenteredAbove = (props: PropsWithChildren<PosProps>) => {
  return (
    <AtPosition
      offset={(ref) => ({
        x: ref.current.offsetWidth / 2,
        y: ref.current.offsetHeight,
      })}
      {...props}
    />
  );
};

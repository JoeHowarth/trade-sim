import React, { ReactChild } from "react";
import { Box, Heading, Level, Button } from "react-bulma-components";

function OverlayWindow({
  children,
  title,
  onClickExit,
}: {
  children: React.ReactNode;
  title: string;
  onClickExit: React.MouseEventHandler<HTMLButtonElement>;
}): JSX.Element {
  return (
    <Box
      style={{
        maxWidth: 400,
        border: "1px solid #DDD",
      }}
    >
      <Level style={{ borderBottom: "2px solid #AAA", paddingBottom: 5 }}>
        <Level.Side>
          <Level.Item>
            <Heading size={3} weight={"normal"}>
              {title}
            </Heading>
          </Level.Item>
        </Level.Side>
        <Level.Side align="right">
          <Level.Item>
            <Button remove onClick={onClickExit}></Button>
          </Level.Item>
        </Level.Side>
      </Level>
      {children}
    </Box>
  );
}

export default OverlayWindow;

import { style } from "@vanilla-extract/css";

export const container = style({
  flex: "1",
  display: "flex",
  gap: "16px",
  alignItems: "center",
  justifyContent: "flex-end",
});

export const statusMessage = style({
  margin: "0px",
});

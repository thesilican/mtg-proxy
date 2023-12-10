import { style } from "@vanilla-extract/css";

export const container = style({
  margin: "auto",
  padding: "16px",
  maxWidth: "800px",
  display: "flex",
  flexDirection: "column",
  gap: "16px",
});

export const title = style({
  margin: "0px",
  textAlign: "center",
});

import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const container = style({
  display: "flex",
  flexDirection: "column",
  gap: "16px",
});

export const separator = style({
  flex: "1",
  margin: "0px",
  backgroundColor: vars.gray5,
  minHeight: "1px",
  border: "none",
});

export const printRow = style({
  display: "flex",
  gap: "8px",
  alignItems: "center",
});

export const statusMessage = style({
  margin: "0px",
});

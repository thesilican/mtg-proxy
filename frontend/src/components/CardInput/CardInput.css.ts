import { style } from "@vanilla-extract/css";
import { vars } from "../../global.css";

export const container = style({
  alignSelf: "center",
  display: "flex",
  gap: "8px",
  position: "relative",
  maxWidth: "600px",
  width: "100%",
});

export const inputField = style({
  flex: "1",
  userSelect: "none",
});

export const autocomplete = style({
  position: "absolute",
  left: "0px",
  right: "0px",
  top: "100%",
  padding: "8px 0px",
  display: "flex",
  alignItems: "start",
  gap: "8px",
  userSelect: "none",
});

export const dropdown = style({
  flex: "1",
  borderRadius: "4px",
  backgroundColor: vars.white,
  boxShadow: vars.shadow,
  display: "flex",
  flexDirection: "column",
  overflow: "auto",
  maxHeight: "204px",
});

export const entry = style({
  padding: "4px",
});

export const active = style({
  backgroundColor: vars.gray2,
});

export const img = style({
  boxShadow: "0 8px 16px rgba(0, 0, 0, .15)",
  borderRadius: "8px",
  width: "146px",
  height: "204px",
});

export const hidden = style({
  visibility: "hidden",
});

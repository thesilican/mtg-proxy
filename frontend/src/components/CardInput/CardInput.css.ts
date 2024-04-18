import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const wrapper = style({
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  padding: "16px",
  justifyContent: "center",
  background: "white",
  position: "sticky",
  top: "0px",
  borderBottom: `1px solid ${vars.gray4}`,
});

export const title = style({
  margin: "0px",
});

export const container = style({
  flex: "1",
  display: "flex",
  gap: "8px",
  position: "relative",
  maxWidth: "600px",
  width: "100%",
});

export const inputField = style({
  flex: "1",
  userSelect: "none",
  borderRadius: "4px",
  border: `1px solid ${vars.gray5}`,
  ":focus-visible": {
    borderColor: vars.blue6,
  },
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
  boxShadow: "0 8px 16px rgba(0, 0, 0, .15)",
  display: "flex",
  flexDirection: "column",
  overflow: "auto",
  maxHeight: "204px",
});

export const entry = style({
  padding: "4px",
});

export const active = style({
  backgroundColor: vars.gray3,
});

export const img = style({
  boxShadow: "0 8px 16px rgba(0, 0, 0, .15)",
  borderRadius: "4px",
  width: "146px",
  height: "204px",
});

export const hidden = style({
  visibility: "hidden",
});

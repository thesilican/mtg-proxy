import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const container = style({
  margin: "auto",
  padding: "24px 40px",
  maxWidth: "850px",
  display: "flex",
  flexDirection: "column",
  gap: "16px",
  "@media": {
    "screen and (min-width: 850px)": {
      margin: "24px auto",
      borderRadius: "8px",
      boxShadow: "1px 5px 10px 0px rgba(0, 0, 0, 0.5)",
      backgroundColor: vars.white,
    },
  },
  zIndex: 2,
});

export const footer = style({
  display: "flex",
  position: "sticky",
  bottom: "0px",
  gap: "16px",
  background: "white",
  padding: "16px 0px",
  borderTop: `1px solid ${vars.gray4}`,
  marginTop: "8px",
});

export const separator = style({
  flex: "1",
  margin: "0px",
  backgroundColor: vars.gray5,
  minHeight: "1px",
  border: "none",
});

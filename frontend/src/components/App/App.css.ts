import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const container = style({
  margin: "auto",
  padding: "0px 24px",
  maxWidth: "850px",
  display: "flex",
  flexDirection: "column",
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
  position: "sticky",
  bottom: "0px",
  background: "white",
  padding: "16px 0px",
  borderTop: `1px solid ${vars.gray4}`,
  display: "flex",
  gap: "8px",
  flexWrap: "wrap",
  zIndex: 1,
});

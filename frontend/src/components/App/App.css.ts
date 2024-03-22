import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const container = style({
  margin: "auto",
  padding: "40px",
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

export const title = style({
  margin: "0px",
  textAlign: "center",
});

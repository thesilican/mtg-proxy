import { style } from "@vanilla-extract/css";
import { vars } from "../../../global.css";

export const dialog = style({
  position: "fixed",
  top: "0px",
  border: "none",
  borderRadius: "8px",
  boxShadow: vars.shadow,
  background: vars.glass,
  backdropFilter: "blur(4px)",
  width: "600px",
  flexDirection: "column",
  padding: "16px",
  gap: "16px",
  selectors: {
    "&[open]": {
      display: "flex",
    },
  },
});

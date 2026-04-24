import { style } from "@vanilla-extract/css";
import { vars } from "../../../global.css";

export const input = style({
  borderRadius: "4px",
  backgroundColor: vars.glass,
  outline: "none",
  ":focus-visible": {
    borderColor: vars.blue6,
  },
});

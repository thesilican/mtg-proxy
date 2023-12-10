import { style } from "@vanilla-extract/css";
import { vars } from "../../../style/global.css";

export const input = style({
  borderRadius: "4px",
  border: `1px solid ${vars.gray5}`,
  outline: "none",
  ":focus-visible": {
    borderColor: vars.blue6,
  },
});

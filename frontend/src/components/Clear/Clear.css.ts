import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const dialog = style({
  width: "200px",
  justifyContent: "center",
  textAlign: "center",
});

export const message = style({
  margin: "0px",
});

export const buttonRow = style({
  display: "flex",
  gap: "16px",
  justifyContent: "center",
});

export const cancel = style({
  backgroundColor: vars.gray6,
  selectors: {
    "&:active:not(:disabled)": {
      backgroundColor: vars.gray7,
    },
  },
});

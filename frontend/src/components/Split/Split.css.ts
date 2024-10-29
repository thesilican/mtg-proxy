import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const dialog = style({
  width: "300px",
  gap: "8px",
});

export const numberInput = style({
  width: "60px",
});

export const helperText = style({
  margin: "0px",
});

export const disabled = style({
  color: vars.gray6,
});

export const buttonRow = style({
  display: "flex",
  justifyContent: "start",
});

export const close = style({
  backgroundColor: vars.gray6,
  selectors: {
    "&:active:not(:disabled)": {
      backgroundColor: vars.gray7,
    },
  },
});

import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const textarea = style({
  width: "100%",
  height: "400px",
  borderRadius: "8px",
});

export const error = style({
  margin: "0px",
});

export const hidden = style({
  display: "none",
});

export const buttonRow = style({
  display: "flex",
  gap: "8px",
});

export const cancel = style({
  backgroundColor: vars.gray6,
  selectors: {
    "&:active:not(:disabled)": {
      backgroundColor: vars.gray7,
    },
  },
});

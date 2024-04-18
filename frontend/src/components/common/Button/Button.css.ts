import { style } from "@vanilla-extract/css";
import { vars } from "../../../style/global.css";

export const button = style({
  userSelect: "none",
  border: "none",
  color: "white",
  borderRadius: "4px",
  backgroundColor: vars.blue5,
  cursor: "pointer",
  transition: "background-color 0.1s ease, transform 0.1s ease",
  selectors: {
    "&:active:not(:disabled)": {
      transform: "translateY(1px)",
      backgroundColor: vars.blue6,
    },
  },
  ":disabled": {
    color: vars.gray4,
  },
});

export const small = style({
  fontSize: "12px",
  padding: "0px 6px",
});

export const secondary = style({
  backgroundColor: vars.gray6,
  selectors: {
    "&:active:not(:disabled)": {
      backgroundColor: vars.gray7,
    },
  },
});

export const danger = style({
  backgroundColor: vars.red5,
  selectors: {
    "&:active:not(:disabled)": {
      backgroundColor: vars.red6,
    },
  },
});

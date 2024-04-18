import { style } from "@vanilla-extract/css";

export const text = style({
  display: "none",
  margin: "0px",
  alignSelf: "center",
  "@media": {
    "screen and (min-width: 850px)": {
      display: "block",
    },
  },
});

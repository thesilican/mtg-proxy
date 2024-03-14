import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const container = style({
  display: "flex",
  flexWrap: "wrap",
  rowGap: "16px",
  columnGap: "8px",
  minHeight: "400px",
});

export const cardStyle = style({
  display: "flex",
  flexDirection: "column",
  gap: "4px",
  padding: "4px",
  borderRadius: "4px",
});

export const top = style({
  display: "flex",
  justifyContent: "end",
  gap: "4px",
});

export const number = style({
  width: "60px",
  fontSize: "12px",
});

export const spacer = style({
  flex: "1",
});

export const flip = style({
  fontSize: "12px",
  padding: "0px 6px",
});

export const remove = style({
  fontSize: "12px",
  padding: "0px 6px",
  backgroundColor: vars.red5,
  selectors: {
    "&:active:not(:disabled)": {
      backgroundColor: vars.red6,
    },
  },
});

export const img = style({
  borderRadius: "6px",
  width: "175px",
  height: "245px",
});

export const bottom = style({});

export const select = style({
  fontSize: "12px",
  width: "175px",
  borderRadius: "4px",
  border: `1px solid ${vars.gray5}`,
  backgroundColor: vars.white,
});

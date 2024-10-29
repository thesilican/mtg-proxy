import { style } from "@vanilla-extract/css";
import { vars } from "../../style/global.css";

export const container = style({
  display: "grid",
  rowGap: "16px",
  columnGap: "8px",
  minHeight: "400px",
  padding: "16px 0px",
  justifyItems: "center",
  gridTemplateColumns: "repeat(1, 1fr)",
  "@media": {
    // 180 * 2 + 8 * 1 + 2 * 24
    "screen and (min-width: 416px)": {
      gridTemplateColumns: "repeat(2, 1fr)",
    },
    // 180 * 3 + 8 * 2 + 2 * 24
    "screen and (min-width: 604px)": {
      gridTemplateColumns: "repeat(3, 1fr)",
    },
    // 180 * 4 + 8 * 3 + 2 * 24
    "screen and (min-width: 792px)": {
      gridTemplateColumns: "repeat(4, 1fr)",
    },
  },
});

export const placeholder = style({
  gridColumn: "1 / -1",
});

export const cardStyle = style({
  width: "180px",
  display: "flex",
  flexDirection: "column",
  gap: "4px",
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

export const img = style({
  borderRadius: "6px",
  width: "180px",
  height: "252px",
});

export const bottom = style({});

export const select = style({
  fontSize: "12px",
  width: "175px",
  borderRadius: "4px",
  border: `1px solid ${vars.gray5}`,
  backgroundColor: vars.white,
});

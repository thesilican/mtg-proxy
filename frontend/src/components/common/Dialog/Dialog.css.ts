import { style } from "@vanilla-extract/css";

export const dialog = style({
  position: "fixed",
  top: "0px",
  border: "none",
  borderRadius: "8px",
  boxShadow: "1px 5px 10px 0px rgba(0, 0, 0, 0.5)",
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

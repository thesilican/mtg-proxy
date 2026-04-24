import { style } from "@vanilla-extract/css";
import { vars } from "../../global.css";

export const app = style({
  margin: "auto",
  maxWidth: "850px",
  minHeight: "100vh",
  display: "flex",
  flexDirection: "column",
  zIndex: 2,
});

export const background = style({
  position: "fixed",
  top: "0",
  bottom: "0",
  left: "0",
  right: "0",
  zIndex: "-1",
  backgroundColor: vars.gray1,
  backgroundSize: "cover",
  backgroundAttachment: "fixed",
  backgroundPosition: "center",
  backgroundImage: `url(/background.jpg)`,
});

export const header = style({
  display: "flex",
  flexDirection: "column",
  gap: "8px",
  position: "sticky",
  top: "0px",
  zIndex: 1,
  padding: "16px",
  "@media": {
    "screen and (min-width: 850px)": {
      top: "8px",
      borderRadius: "8px",
    },
  },
  boxShadow: vars.shadow,
  alignItems: "center",
  background: vars.glass,
  backdropFilter: "blur(4px)",
});

export const titleBox = style({
  display: "flex",
  flexDirection: "column",
  textAlign: "center",
});

export const footer = style({
  display: "flex",
  flexDirection: "column",
  gap: "8px",
  position: "sticky",
  bottom: "0px",
  zIndex: 1,
  padding: "16px",
  background: vars.glass,
  backdropFilter: "blur(4px)",
  "@media": {
    "screen and (min-width: 850px)": {
      bottom: "8px",
      borderRadius: "8px",
    },
  },
  boxShadow: vars.shadow,
});

export const row = style({
  display: "flex",
  gap: "8px",
  flexWrap: "wrap",
});

export const row2 = style({
  display: "flex",
  gap: "8px",
  flexWrap: "wrap",
  justifyContent: "space-between",
  alignItems: "end",
});

export const artistCredit = style({
  fontSize: "12px",
});

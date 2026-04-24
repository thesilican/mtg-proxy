import { createTheme, globalStyle } from "@vanilla-extract/css";

export const [defaultTheme, vars] = createTheme({
  white: "#fff",
  gray1: "#f8f9fa",
  gray2: "#e9ecef",
  gray3: "#dee2e6",
  gray4: "#ced4da",
  gray5: "#adb5bd",
  gray6: "#6c757d",
  gray7: "#495057",
  gray8: "#343a40",
  gray9: "#212529",
  black: "#000",
  blue5: "#0d6efd",
  blue6: "#0a58ca",
  red5: "#dc3545",
  red6: "#b02a37",
  glass: "#fffd",
  shadow: "0px 2px 4px 0px rgba(0, 0, 0, 0.4)",
});

globalStyle("body", {
  fontFamily: "Rubik, sans-serif",
});

globalStyle("p, h1", {
  margin: "0px",
});

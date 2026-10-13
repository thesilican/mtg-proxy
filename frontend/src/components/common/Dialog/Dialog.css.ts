import { keyframes, style } from "@vanilla-extract/css";
import { vars } from "../../../global.css";

const fadeIn = keyframes({
	"0%": { opacity: 0 },
	"100%": { opacity: 1 },
});

export const dialog = style({
	position: "fixed",
	top: "0px",
	border: "none",
	borderRadius: "8px",
	boxShadow: vars.shadow,
	background: vars.glass,
	backdropFilter: "blur(4px)",
	width: "600px",
	flexDirection: "column",
	padding: "16px",
	gap: "16px",
	selectors: {
		"&[open]": {
			display: "flex",
		},
	},
	animation: `${fadeIn} 0.1s ease`,
});

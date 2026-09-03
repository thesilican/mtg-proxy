import cn from "classnames";
import type { MouseEventHandler, ReactNode } from "react";
import * as styles from "./Button.css";

type Props = {
	className?: string;
	type?: "submit" | "reset" | "button";
	title?: string;
	children?: ReactNode;
	disabled?: boolean;
	variant?: "normal" | "secondary" | "danger";
	size?: "normal" | "small";
	onClick?: MouseEventHandler<HTMLButtonElement>;
};

export function Button(props: Props) {
	return (
		<button
			className={cn(
				styles.button,
				props.variant === "secondary" && styles.secondary,
				props.variant === "danger" && styles.danger,
				props.size === "small" && styles.small,
				props.className,
			)}
			type={props.type}
			title={props.title}
			disabled={props.disabled}
			onClick={props.onClick}
		>
			{props.children}
		</button>
	);
}

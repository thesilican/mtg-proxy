import { MouseEventHandler, ReactNode } from "react";
import { button, danger, secondary, small } from "./Button.css";
import cn from "classnames";

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
        button,
        props.variant === "secondary" && secondary,
        props.variant === "danger" && danger,
        props.size === "small" && small,
        props.className
      )}
      type={props.type}
      title={props.title}
      children={props.children}
      disabled={props.disabled}
      onClick={props.onClick}
    />
  );
}

import { MouseEventHandler, ReactNode } from "react";
import { button } from "./Button.css";
import cn from "classnames";

type Props = {
  className?: string;
  type?: "submit" | "reset" | "button";
  title?: string;
  children?: ReactNode;
  disabled?: boolean;
  onClick?: MouseEventHandler<HTMLButtonElement>;
};

export function Button(props: Props) {
  return (
    <button
      className={cn(button, props.className)}
      type={props.type}
      title={props.title}
      children={props.children}
      disabled={props.disabled}
      onClick={props.onClick}
    />
  );
}

import {
  ChangeEventHandler,
  HTMLInputTypeAttribute,
  KeyboardEventHandler,
} from "react";
import cn from "classnames";
import { input } from "./Input.css";

type Props = {
  className?: string;
  type?: HTMLInputTypeAttribute;
  value?: string | number | readonly string[];
  min?: string | number;
  max?: string | number;
  placeholder?: string;
  autoFocus?: boolean;
  onChange?: ChangeEventHandler<HTMLInputElement>;
  onKeyDown?: KeyboardEventHandler<HTMLInputElement>;
};

export function Input(props: Props) {
  return (
    <input
      className={cn(input, props.className)}
      type={props.type}
      value={props.value}
      min={props.min}
      max={props.max}
      placeholder={props.placeholder}
      autoFocus={props.autoFocus}
      onChange={props.onChange}
      onKeyDown={props.onKeyDown}
    />
  );
}

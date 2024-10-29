import {
  ChangeEventHandler,
  HTMLInputTypeAttribute,
  KeyboardEventHandler,
  Ref,
} from "react";
import cn from "classnames";
import { input } from "./Input.css";
import React from "react";

type Props = {
  className?: string;
  type?: HTMLInputTypeAttribute;
  value?: string | number | readonly string[];
  min?: string | number;
  max?: string | number;
  placeholder?: string;
  autoFocus?: boolean;
  disabled?: boolean;
  onChange?: ChangeEventHandler<HTMLInputElement>;
  onKeyDown?: KeyboardEventHandler<HTMLInputElement>;
};

export const Input = React.forwardRef(function Input(
  props: Props,
  ref?: Ref<HTMLInputElement>
) {
  return <input {...props} ref={ref} className={cn(input, props.className)} />;
});

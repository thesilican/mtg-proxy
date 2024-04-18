import cn from "classnames";
import { ReactNode, SyntheticEvent, useEffect, useRef } from "react";
import { dialog } from "./Dialog.css";

type DialogProps = {
  open: boolean;
  className?: string;
  children?: ReactNode;
  onClose?: () => void;
};

export function Dialog(props: DialogProps) {
  const ref = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }
    if (props.open && !ref.current.open) {
      ref.current.showModal();
    }
    if (!props.open && ref.current.open) {
      ref.current.close();
    }
  }, [props.open]);

  const handleClose = (e: SyntheticEvent<HTMLDialogElement>) => {
    if (e.currentTarget.open) {
      props.onClose?.();
    }
  };

  return (
    <dialog
      ref={ref}
      className={cn(dialog, props.className)}
      onClose={handleClose}
      onCancel={handleClose}
    >
      {props.children}
    </dialog>
  );
}

import cn from "classnames";
import { type ReactNode, useEffect, useRef } from "react";
import * as styles from "./Dialog.css";

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

	const handleClose = () => {
		props.onClose?.();
	};

	return (
		<dialog
			ref={ref}
			className={cn(styles.dialog, props.className)}
			onClose={handleClose}
			onCancel={handleClose}
		>
			{props.children}
		</dialog>
	);
}

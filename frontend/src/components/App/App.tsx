import cn from "classnames";
import { useLayoutEffect } from "react";
import * as globalStyles from "../../global.css";
import { useAppSelector } from "../../state";
import { CardInput } from "../CardInput/CardInput";
import { CardPreview } from "../CardPreview/CardPreview";
import { Clear } from "../Clear/Clear";
import { Export } from "../Export/Export";
import { Import } from "../Import/Import";
import { LocalStorage } from "../LocalStorage/LocalStorage";
import { Print } from "../Print/Print";
import { Split } from "../Split/Split";
import { Status } from "../Status/Status";
import * as styles from "./App.css";

export function App() {
	// Change background blur effect based on whether there are cards to display
	const hasCards = useAppSelector((state) => state.print.cards.length > 0);
	useLayoutEffect(() => {
		const classes = document.body.classList;
		if (hasCards) {
			classes.add(globalStyles.blurred);
		} else {
			classes.remove(globalStyles.blurred);
		}
		return () => classes.remove(globalStyles.blurred);
	}, [hasCards]);

	return (
		<div className={cn(styles.app, globalStyles.defaultTheme)}>
			<div className={styles.header}>
				<div className={styles.titleBox}>
					<h1>MTG Proxy Maker</h1>
					<p>
						By <a href="https://thesilican.com">Bryan Chen</a>
					</p>
				</div>
				<CardInput />
			</div>
			<CardPreview />
			<div className={styles.footer}>
				<div className={styles.row}>
					<Import />
					<Export />
					<Split />
					<Clear />
					<Print />
				</div>
				<div className={styles.row2}>
					<Status />
					<p className={styles.artistCredit}>
						Background:{" "}
						<a
							href="https://algenpfleger.artstation.com/projects/GaWG4d"
							target="_blank"
							rel="noopener"
						>
							Island
						</a>{" "}
						by Johannes Voss
					</p>
				</div>
			</div>
			<LocalStorage />
		</div>
	);
}

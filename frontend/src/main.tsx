import ReactDOM from "react-dom/client";
import { Provider } from "react-redux";
import "sanitize.css";
import "sanitize.css/forms.css";
import { App } from "./components/App/App";
import { store } from "./state";

const root = document.getElementById("root") as HTMLDivElement;

ReactDOM.createRoot(root).render(
	<Provider store={store}>
		<App />
	</Provider>,
);

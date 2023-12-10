import ReactDOM from "react-dom/client";
import "sanitize.css";
import "sanitize.css/typography.css";
import "sanitize.css/forms.css";
import { App } from "./components/App/App";
import { Provider } from "react-redux";
import { store } from "./state";

const root = document.getElementById("root") as HTMLDivElement;

ReactDOM.createRoot(root).render(
  <Provider store={store}>
    <App />
  </Provider>
);

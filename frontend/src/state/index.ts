import { configureStore } from "@reduxjs/toolkit";
import {
	type TypedUseSelectorHook,
	useDispatch,
	useSelector,
} from "react-redux";
import { scryfallApi } from "./api";
import { printSlice } from "./print";

export const store = configureStore({
	reducer: {
		print: printSlice.reducer,
		api: scryfallApi.reducer,
	},
	middleware: (getDefaultMiddleware) =>
		getDefaultMiddleware().concat(scryfallApi.middleware),
});

export type AppState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export const useAppSelector: TypedUseSelectorHook<AppState> = useSelector;
export const useAppDispatch: () => AppDispatch = useDispatch;

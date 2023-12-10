import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export type UiState = {
  snackbar: Snackbar;
};

export type Snackbar = {
  message: string;
  status: "error" | "info";
  shown: boolean;
};

const initialState: UiState = {
  snackbar: {
    message: "",
    status: "info",
    shown: false,
  },
};

export const uiSlice = createSlice({
  name: "ui",
  initialState,
  reducers: {
    setSnackbar(state, action: PayloadAction<Partial<Snackbar>>) {
      state.snackbar = {
        ...state.snackbar,
        ...action.payload,
      };
    },
  },
});

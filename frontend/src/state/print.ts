import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export type Card = {
  id: string;
  name: string;
  quantity: number;
  face: "front" | "back";
};

export type PrintState = {
  cards: Card[];
  // How many pages per pdf
  split: number | null;
};

const initialState: PrintState = {
  cards: [],
  split: null,
};

export const printSlice = createSlice({
  name: "print",
  initialState,
  reducers: {
    load(_, action: PayloadAction<PrintState>) {
      return action.payload;
    },
    add(state, action: PayloadAction<Card>) {
      state.cards.push(action.payload);
    },
    update(state, action: PayloadAction<{ idx: number; card: Partial<Card> }>) {
      if (action.payload.idx < 0 || action.payload.idx >= state.cards.length) {
        return;
      }
      state.cards[action.payload.idx] = {
        ...state.cards[action.payload.idx],
        ...action.payload.card,
      };
    },
    remove(state, action: PayloadAction<number>) {
      state.cards.splice(action.payload, 1);
    },
    clear(state) {
      state.cards = [];
    },
    setSplit(state, action: PayloadAction<number | null>) {
      state.split = action.payload;
    },
  },
});

export const printAction = printSlice.actions;

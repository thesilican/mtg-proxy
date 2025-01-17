import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export type Card = {
  /** Scryfall card id */
  id: string;
  /** Scryfall card name */
  name: string;
  /** Quantity of cards */
  quantity: number;
  /** Card face (0 = front, 1 = back) */
  face: number;
};

export type PrintState = {
  cards: Card[];
  /** Split pdf feature, how many pages per pdf */
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

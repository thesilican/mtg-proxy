import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export type Card = {
  name: string;
  quantity: number;
  variant: number;
  face: number;
  ids: string[];
};

export type PrintState = {
  cards: Card[];
};

const initialState: PrintState = {
  cards: [],
};

export const printSlice = createSlice({
  name: "print",
  initialState,
  reducers: {
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
    load(state, action: PayloadAction<Card[]>) {
      state.cards = action.payload;
    },
  },
});

export const printAction = printSlice.actions;

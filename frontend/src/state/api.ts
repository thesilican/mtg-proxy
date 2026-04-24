import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

type ApiCardImages = {
  front_jpg: string;
  back_jpg: string | null;
  front_png: string;
  back_png: string | null;
};

export type ApiCard = {
  id: string;
  name: string;
  flavor_name: string | null;
  set: string;
  set_name: string;
  collector_number: string;
  images: ApiCardImages;
  preferred: boolean;
};

export type ImportCard = {
  name: string;
  set: string | null;
  collector_number: string | null;
};

type ApiImportCard =
  | {
      success: true;
      card: ApiCard;
    }
  | {
      success: false;
      message: string;
    };

export const scryfallApi = createApi({
  reducerPath: "api",
  baseQuery: fetchBaseQuery({ baseUrl: import.meta.env.BASE_URL }),
  endpoints: (builder) => ({
    search: builder.query<{ cards: ApiCard[] }, { q: string }>({
      query: ({ q }) => {
        return `/api/search?q=${encodeURIComponent(q)}`;
      },
    }),
    cards: builder.query<
      { cards: ApiCard[] },
      { name: string } | { ids: string[] }
    >({
      query: (params) => {
        if ("name" in params) {
          return `/api/cards?name=${encodeURIComponent(params.name)}`;
        } else {
          return `/api/cards?ids=${encodeURIComponent(params.ids.join(","))}`;
        }
      },
    }),
    import: builder.query<
      { results: ApiImportCard[] },
      { cards: ImportCard[] }
    >({
      query: (body) => ({
        method: "POST",
        url: "/api/import",
        body,
      }),
    }),
  }),
});

export const {
  useSearchQuery,
  useLazySearchQuery,
  useCardsQuery,
  useLazyCardsQuery,
  useImportQuery,
  useLazyImportQuery,
} = scryfallApi;

export function isDfc(card: ApiCard): card is ApiCard & {
  images: ApiCardImages & { back_jpg: string; back_png: string };
} {
  return card.images.back_jpg !== null && card.images.back_png !== null;
}

export function getPreferredCard(cards: ApiCard[]): ApiCard {
  for (const card of cards) {
    if (card.preferred) {
      return card;
    }
  }
  return cards[0];
}

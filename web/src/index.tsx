import React from "react";
import ReactDOM from "react-dom/client";
import reportWebVitals from "./reportWebVitals";
import { createHashRouter, redirect, RouterProvider } from "react-router-dom";
import Root from "./routes/root";
import ErrorPage from "./error";
import Vote from "./routes/vote/vote";
import QrCodes from "./routes/qr-codes/qr-codes";
import { ChakraProvider, extendTheme } from "@chakra-ui/react";
import { getLocation, getLocations } from "./data/api/location";
import Voted from "./routes/voted/voted";
import {
  addVotedForLocation,
  hasAlreadyVotedForLocation,
} from "./data/local/voted-locations";
import "./index.css";
import { submitVote } from "./data/api/vote";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

const theme = extendTheme({
  brand: {
    900: "#1a365d",
    800: "#153e75",
    700: "#2a69ac",
  },
});

const router = createHashRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "vote/:locationId",
        element: <Vote />,
        loader: async (params: any) => {
          const locationId = params.params.locationId;
          const agrees = new URL(params.request.url).searchParams.get("agrees");

          if (hasAlreadyVotedForLocation(locationId)) {
            return redirect("/voted");
          }

          if (agrees != null) {
            await submitVote({
              comment: null,
              location_id: locationId,
              agrees: !!(agrees as unknown as boolean),
            });
            addVotedForLocation(locationId);
            return redirect("/voted");
          }

          return {
            location: await getLocation(locationId),
          };
        },
      },
      {
        path: "voted",
        element: <Voted />,
      },
      {
        path: "qrCodes",
        element: <QrCodes />,
        loader: async () => ({
          locations: await getLocations(),
        }),
      },
    ],
  },
]);

root.render(
  <React.StrictMode>
    <ChakraProvider theme={theme}>
      <RouterProvider router={router} />
    </ChakraProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();

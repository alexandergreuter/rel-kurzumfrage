import { DownloadIcon } from "@chakra-ui/icons";
import {
  Button,
  Table,
  TableContainer,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react";
import { useLoaderData } from "react-router-dom";
import { Location } from "../../data/dto/location";
import QRCodeStyling from "qr-code-styling";

const frontendUrl = "https://kurzumfrage.rel-suhr.ch";

export default function QrCodes() {
  const { locations } = useLoaderData() as { locations: Location[] };

  return (
    <TableContainer>
      <Table variant="striped">
        <Thead>
          <Tr>
            <Th>Titel</Th>
            <Th>Frage</Th>
            <Th>Ja</Th>
            <Th>Nein</Th>
          </Tr>
        </Thead>
        <Tbody>
          {locations.map((it) => (
            <LocationRow location={it} />
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  );
}

function LocationRow({ location }: { location: Location }) {
  function handleDownloadClick(agrees: boolean) {
    new QRCodeStyling({
      width: 300,
      height: 300,
      data: frontendUrl + "/vote/" + location.id + "?agrees=" + agrees,
      image: agrees ? "/ja.png" : "/nein.png",
      dotsOptions: {
        type: "rounded"
      },
      cornersDotOptions:{
        type: undefined
      },
      imageOptions: {
        margin: 10,
        imageSize: agrees ? 0.25 : 0.5
      },
    }).download({
      name: encodeURIComponent(
        location.title +
          " - " +
          location.prompt +
          " - " +
          (agrees ? "ja" : "nein")
      ),
    });
  }

  return (
    <Tr>
      <Td>{location.title}</Td>
      <Td>{location.prompt}</Td>
      <Td>
        <Button onClick={() => handleDownloadClick(true)}>
          <DownloadIcon></DownloadIcon>
        </Button>
      </Td>
      <Td>
        <Button onClick={() => handleDownloadClick(false)}>
          <DownloadIcon></DownloadIcon>
        </Button>
      </Td>
    </Tr>
  );
}

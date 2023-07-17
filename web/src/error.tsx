import { VStack, Text, Container, Center } from "@chakra-ui/react";
import { useRouteError } from "react-router-dom";

export default function ErrorPage() {
  const error = useRouteError() as any;
  console.error(error);

  return (
    <Container h="100%" p="4">
      <Center h="100%">
        <VStack>
          <Text>Oops!</Text>
          <Text>Sorry, an unexpected error has occurred.</Text>
          <Text>{error.statusText || error.message}</Text>
        </VStack>
      </Center>
    </Container>
  );
}

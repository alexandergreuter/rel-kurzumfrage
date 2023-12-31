import {
  CheckIcon,
  ExternalLinkIcon,
} from "@chakra-ui/icons";
import {
  Center,
  Container,
  Link,
  Text,
  VStack,
} from "@chakra-ui/react";

export default function Voted() {
  return (
    <Container h="100%" p="4">
      <Center h="100%">
        <VStack spacing="3">
          <CheckIcon fontSize="xl"></CheckIcon>
          <Text fontSize="xl" mt="3" textAlign="center">
            Erfolgreich gesendet, vielen Dank für dein Feeback!
          </Text>
          <Link href="https://suhr.e-mitwirkung.ch/de/rel/participant/survey-document-groups/4187" isExternal>
            Ausführliche Umfrage <ExternalLinkIcon mx="2px" />
          </Link>
          <Link href="http://www.rel-suhr.ch/" isExternal>
            Mehr zum REL Suhr <ExternalLinkIcon mx="2px" />
          </Link>
        </VStack>
      </Center>
    </Container>
  );
}

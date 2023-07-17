import { Box, Container, Flex, Text } from "@chakra-ui/react";
import { Outlet } from "react-router";

export default function Root() {
  return (
    <Flex flexDirection="column" minH="100%">
      <Box flexGrow="1" overflow="auto">
        <Outlet />
      </Box>
      <Box backgroundColor="gray.100">
        <Container p="4">
          <Text textAlign="center">Kurzumfrage - REL Suhr</Text>
        </Container>
      </Box>
    </Flex>
  );
}

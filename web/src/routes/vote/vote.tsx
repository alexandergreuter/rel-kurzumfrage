import {
  VStack,
  Text,
  FormControl,
  FormLabel,
  Button,
  RadioGroup,
  HStack,
  Radio,
  Textarea,
  Container,
  Center,
} from "@chakra-ui/react";
import { useLoaderData, useNavigate, useSearchParams } from "react-router-dom";
import { Location } from "../../data/dto/location";
import { Field, FieldProps, Formik } from "formik";
import { submitVote } from "../../data/api/vote";
import { addVotedForLocation } from "../../data/local/voted-locations";

export default function Vote() {
  let [searchParams] = useSearchParams();
  const { location } = useLoaderData() as { location: Location };
  const navigate = useNavigate();

  return (
    <Container p="4">
      <Formik
        initialValues={{
          agrees: searchParams.get("agrees") ?? "false",
          comment: "",
        }}
        onSubmit={(values) => {
          submitVote({
            ...values,
            location_id: location.id,
            agrees: !!(values.agrees as unknown as boolean),
          });
          addVotedForLocation(location.id);
          navigate("/voted");
        }}
      >
        {({ handleSubmit, errors, touched, isSubmitting }) => (
          <form onSubmit={handleSubmit}>
            <VStack align="start" spacing="3">
              <Text fontSize="xl">{location.title}</Text>

              <FormControl as="fieldset" isRequired width="full">
                <FormLabel as="legend">{location.prompt}</FormLabel>
                <Field name="agrees">
                  {({ field: { onChange, value } }: FieldProps<string>) => (
                    <RadioGroup
                      value={value}
                      onChange={(it) => onChange("agrees")(it)}
                    >
                      <HStack spacing="24px">
                        <Radio value="true">Ja</Radio>
                        <Radio value="false">Nein</Radio>
                      </HStack>
                    </RadioGroup>
                  )}
                </Field>
              </FormControl>
              <FormControl width="full">
                <FormLabel>Kommentar</FormLabel>
                <Field as={Textarea} name="comment" />
              </FormControl>
              <Button isLoading={isSubmitting} type="submit" width="full">
                Submit
              </Button>
            </VStack>
          </form>
        )}
      </Formik>
    </Container>
  );
}

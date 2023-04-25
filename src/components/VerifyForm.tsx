import { useEffect, useState } from "react";
import { Input, Stack, Text, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { notifications } from "@mantine/notifications";
import * as hm from '../lib/wasm/hammster.js'

export const VerifyForm = () => {
  const [hasProof, setHasProof] = useState(false);

  // Check if we have a proof saved to localStorage yet
  useEffect(() => {
    const proof = localStorage.getItem("proof");
    if (proof) {
      setHasProof(true);
      return;
    }
  }, []);

  const form = useForm({
    initialValues: {
      hammingDist: '',
    },
    validate: {
      hammingDist: (value) => {
        const parsedValue = parseInt(value);
        if (isNaN(parsedValue)) {
          return 'Must be a number';
        }
        if (parsedValue < 0 || parsedValue > 8) {
          return 'Must be between 0 and 8';
        }
        return null;
      },
    },
  })

  const submit = async (values: any) => {
    console.log(values);

    // Get setup params from localStorage
    const paramsString = localStorage.getItem("setupParams");
    if (!paramsString) {
      console.log("No params found");
      return;
    }
    const params = Uint8Array.from((paramsString as string).split(',').map((x: string) => parseInt(x)));

    // Get proof from localStorage
    const proofString = localStorage.getItem("proof");
    if (!proofString) {
      console.log("No proof found");
      return;
    }
    const proof = Uint8Array.from((proofString as string).split(',').map((x: string) => parseInt(x)));

    // Set up hammster wasm & verify the proof
    await hm.default();
    const result = hm.proof_verify(params, values.hammingDist, proof);
    
    if (result) {
      notifications.show({
        title: "Success!",
        message: "Proof verified successfully!",
        color: "green",
      });
    } else {
      notifications.show({
        title: "Error",
        message: `Proof with hamming distance of ${values.hammingDist} failed to verify`,
        color: "red",
      });
    }
  }

  return (
    <form onSubmit={form.onSubmit(submit)}>
      <Stack align='center' w='100%' style={{
        backgroundColor: "rgba(200,230,255,0.25)",
        backdropFilter: 'blur(8px)',
        border: "2px solid #eee",
        borderRadius: "8px",
        padding: "16px",
        boxShadow: "0px 12px 36px rgba(0, 0, 0, 0.2)",
      }}>
        <Text>
          Input the hamming distance from the Prove section.
        </Text>
        <TextInput 
          label="Hamming Distance" 
          placeholder="[0,8]" 
          styles={
            {
              input: {
                backgroundColor: '#def',
                color: '#345',
                fontWeight: 400,
                fontSize: '0.95rem',
              },
            }
          }
          {...form.getInputProps('hammingDist')} 
        />
        <Input 
          type="submit" 
          value="Verify Proof" 
          disabled={!hasProof || form.values.hammingDist.length !== 1} 
        />
      </Stack>
    </form>
  )
}
import { useState } from "react"
import { Group, Input, Stack, Text } from "@mantine/core"
import { BinaryInput } from "./BinaryInput"
import { useForm } from "@mantine/form"
import { HammingDistance } from "./HammingDistance"
import { notifications } from '@mantine/notifications';
import * as hm from '../lib/wasm/hammster.js'

export const ProveForm = () => {
  const [proof, setProof] = useState<Uint8Array>(new Uint8Array());

  const form = useForm({
    initialValues: {
      input0: '',
      input1: '',
    },
  })

  const submit = async (values: any) => {
    console.log(values);
    await hm.default();

    // Run setup
    const params = hm.setup_params();
    try {
      localStorage.setItem("setupParams", params.toString());
    } catch (err) {
      console.log(err);
      notifications.show({
        title: "Error",
        message: "Failed to save setup params",
        color: "red",
      })
      return;
    }

    // Parse inputs
    const input0arr = values.input0.split('').map((x: string) => parseInt(x, 2));
    const input1arr = values.input1.split('').map((x: string) => parseInt(x, 2));
    
    // Generate the proof
    console.log("generating proof");
    const proof = hm.proof_generate(input0arr, input1arr, params);
    if (proof.length === 0) {
      notifications.show({
        title: "Error",
        message: "Failed to generate proof",
        color: "red",
      })
      return;
    }

    setProof(proof);

    // Save proof to localStorage
    try {
      localStorage.setItem("proof", proof.toString());
      console.log("saved to localStorage");
    } catch (err) {
      console.log(err);
      notifications.show({
        title: "Error",
        message: "Failed to save proof",
        color: "red",
      })
      return;
    }

    // Show notification to user
    notifications.show({
      title: "Success!",
      message: "Setup params & proof saved to localStorage",
      color: "green",
    });
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
          Input two 8-bit binary values below. Remember the calculated hamming distance value for the Verify section.
        </Text>
        <BinaryInput inputNum={0} form={form} />
        <BinaryInput inputNum={1} form={form} />
        <HammingDistance form={form} />
        <Group>
          <Input 
            type="submit" 
            value="Generate & Save Proof" 
            disabled={form.values.input0.length !== 8 || form.values.input1.length !== 8} 
          />
        </Group>
      </Stack>
    </form>
  )
}
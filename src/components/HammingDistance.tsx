import { Text } from "@mantine/core"

export const HammingDistance = (props: any) => {
  if (props.form === undefined) {
    return null;
  }

  const calculateHammingDistance = () => {    
    let input0 = props.form.values.input0;
    if (input0.length < 8) {
      input0 = input0.padStart(8, '0');
    }
    let input1 = props.form.values.input1;
    if (input1.length < 8) {
      input1 = input1.padStart(8, '0');
    }

    const input0arr = input0.split('').map((x: string) => parseInt(x, 2));
    const input1arr = input1.split('').map((x: string) => parseInt(x, 2));

    let hammingDistance = 0;
    for (let i = 0; i < input0arr.length; i++) {
      if (input0arr[i] !== input1arr[i]) {
        hammingDistance++;
      }
    }
    return hammingDistance;
  }

  return (
    <Text align="center">
      Hamming distance: <b>{calculateHammingDistance()}</b> 
      <Text fz="xs">
        (remember this number!)
      </Text>
    </Text>
  )
}
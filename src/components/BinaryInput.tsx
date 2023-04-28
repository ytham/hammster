import { Group, PinInput, Text } from "@mantine/core"

export const BinaryInput = (props: any) => {
  return (
    <Group>
      <Text>
        Input {props.inputNum}
      </Text>
      <PinInput type={/^[0-1]+/} length={8} placeholder="0" styles={
          {
            input: {
              backgroundColor: '#def',
              color: '#345',
              fontWeight: 400,
              fontSize: '1.05rem',
            },
          }
        } 
        {...props.form.getInputProps(`input${props.inputNum}`)}
      />
    </Group>
  )
}
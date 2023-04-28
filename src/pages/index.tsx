import Head from 'next/head'
import { Grid, SegmentedControl, Space, Stack, Text, Title } from '@mantine/core'
import Link from 'next/link'
import { ProveForm } from '@/components/ProveForm'
import { useState } from 'react'
import { VerifyForm } from '@/components/VerifyForm'

export default function Home() {
  const [page, setPage] = useState("prove");

  const renderProveOrVerify = () => {
    // Renders the page based on the SegmentedControl selection
    if (page === "verify") { 
      return <VerifyForm />;
    }
    return <ProveForm />
  }

  return (
    <>
      <Head>
        <title>Hammster</title>
        <meta name="description" content="Hammster, the hamming distance ZK app." />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Stack align='center' justify='center' mih="100vh" style={{
        background: 'url(/hammster-corner.png)',
        backgroundRepeat: 'no-repeat',
        backgroundAttachment: 'fixed',
        backgroundPosition: 'right bottom',
        backgroundSize: '50%',
        padding: "0 20px",
      }}>
        <Grid justify='center'>
          <Grid.Col xs={10} sm={8} md={6}>
            <Stack align='center' spacing="xl">
              <Title order={1} ff={"helvetica neue"}>
                Hammster
              </Title>
              <Text>
                Hammster is written in <Link href="https://halo2.dev/">Halo2</Link>. It takes two 8-length vector inputs of 
                binary digits and their <Link href="https://en.wikipedia.org/wiki/Hamming_distance">Hamming distance</Link> and 
                generates a proof that the two inputs are the claimed hamming distance away from each other. Please note that 
                this currently does not work on mobile.
              </Text>
              <SegmentedControl 
                value={page}
                onChange={setPage}
                data={[
                  { value: "prove", label: "Prove" },
                  { value: "verify", label: "Verify" },
                ]}
              />
              { renderProveOrVerify() }
            </Stack>
          </Grid.Col>
        </Grid>
        
        <Space h="16vh" />
        
        <Stack spacing={0} style={{
          position: "fixed",
          left: "8px",
          bottom: "8px",
          padding: "8px 20px",
          borderRadius: "8px",
          boxShadow: "0px 6px 12px rgba(0, 0, 0, 0.1)",
        }}>
          <Text>
            <Link href="https://medium.com/@yujiangtham/building-a-zero-knowledge-web-app-with-halo-2-and-wasm-part-1-80858c8d16ee">Tutorial</Link>
          </Text>
          <Text>
            <Link href="https://github.com/ytham/hammster">Github</Link>
          </Text>
        </Stack>
      </Stack>
    </>
  )
}

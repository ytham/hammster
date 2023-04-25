import Head from 'next/head'
import { Grid, SegmentedControl, Space, Stack, Text, Title } from '@mantine/core'
import Link from 'next/link'
import { ProveForm } from '@/components/ProveForm'
import { useState } from 'react'
import { VerifyForm } from '@/components/VerifyForm'

export default function Home() {
  const [page, setPage] = useState("prove");

  const renderProveOrVerify = () => {
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
                generates a proof that the two inputs are the claimed hamming distance away from each other. 
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
      </Stack>
    </>
  )
}

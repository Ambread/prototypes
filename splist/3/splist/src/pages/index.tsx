import type { NextPage } from 'next';
import { trpc, useMount } from '../utils/hooks';
import { useLocalStorage } from '@mantine/hooks';
import { Messages } from '../components/Messages';
import {
    AppShell,
    Header,
    Navbar,
    TextInput,
    Title,
    Text,
    Alert,
    Stack,
} from '@mantine/core';
import { AlertCircle } from 'tabler-icons-react';

const Home: NextPage = () => {
    const [name, setName] = useLocalStorage({
        key: 'name',
        defaultValue: '',
    });

    const login = trpc.useMutation(['login']);

    useMount(() => {
        login.mutate({ name });
    });

    return (
        <AppShell
            fixed
            style={{ height: '100%', width: '100%' }}
            navbar={
                <Navbar p="md" width={{ lg: 300 }}>
                    <Navbar.Section grow>
                        <Text>Cool navbar</Text>
                    </Navbar.Section>
                    <Navbar.Section>
                        <TextInput
                            label="Username"
                            value={name}
                            onChange={(e) => {
                                setName(e.target.value);
                            }}
                            onKeyDown={(e) => {
                                if (e.key !== 'Enter') {
                                    return;
                                }
                                login.mutate({ name });
                            }}
                        />
                    </Navbar.Section>
                </Navbar>
            }
            header={
                <Header height={70} p="md">
                    <Title>Splist</Title>
                </Header>
            }
        >
            <Stack justify="flex-end" style={{ height: '100%' }}>
                {login.data ? (
                    <Messages />
                ) : (
                    <Alert
                        icon={<AlertCircle size={16} />}
                        color="red"
                        title="Unauthorized"
                    >
                        You need to log in
                    </Alert>
                )}
            </Stack>
        </AppShell>
    );
};

export default Home;

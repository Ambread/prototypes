import type { NextPage } from 'next';
import { trpc, useMount } from '../utils/hooks';
import { useLocalStorage } from '@mantine/hooks';
import { Messages } from '../components/Messages';
import {
    AppShell,
    Footer,
    Header,
    Navbar,
    TextInput,
    Title,
    Text,
} from '@mantine/core';

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
            navbar={
                <Navbar p="md" width={{ sm: 200, lg: 300 }}>
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
            {login.data ? <Messages /> : <p>Logged out</p>}
        </AppShell>
    );
};

export default Home;
